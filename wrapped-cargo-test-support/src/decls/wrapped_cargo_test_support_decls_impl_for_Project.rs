use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Project {
    /// Copy the test project from a fixed state
    pub fn from_template(template_path: impl AsRef<Path>) -> Self {
        let root = paths::root();
        let project_root = root.join("case");
        snapbox::dir::copy_template(template_path.as_ref(), &project_root).unwrap();
        Self { root: project_root }
    }
    /// Root of the project
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo`
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    /// Project's target dir
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target`
    pub fn build_dir(&self) -> PathBuf {
        self.root().join("target")
    }
    /// Project's debug dir
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target/debug`
    pub fn target_debug_dir(&self) -> PathBuf {
        self.build_dir().join("debug")
    }
    /// File url for root
    ///
    /// ex: `file://$CARGO_TARGET_TMPDIR/cit/t0/foo`
    pub fn url(&self) -> Url {
        use paths::CargoPathExt;
        self.root().to_url()
    }
    /// Path to an example built as a library.
    ///
    /// `kind` should be one of: "lib", "rlib", "staticlib", "dylib", "proc-macro"
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target/debug/examples/libex.rlib`
    pub fn example_lib(&self, name: &str, kind: &str) -> PathBuf {
        self.target_debug_dir()
            .join("examples")
            .join(paths::get_lib_filename(name, kind))
    }
    /// Path to a dynamic library.
    /// ex: `/path/to/cargo/target/cit/t0/foo/target/debug/examples/libex.dylib`
    pub fn dylib(&self, name: &str) -> PathBuf {
        self.target_debug_dir()
            .join(
                format!("{}{name}{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX),
            )
    }
    /// Path to a debug binary.
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target/debug/foo`
    pub fn bin(&self, b: &str) -> PathBuf {
        self.build_dir().join("debug").join(&format!("{}{}", b, env::consts::EXE_SUFFIX))
    }
    /// Path to a release binary.
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target/release/foo`
    pub fn release_bin(&self, b: &str) -> PathBuf {
        self.build_dir()
            .join("release")
            .join(&format!("{}{}", b, env::consts::EXE_SUFFIX))
    }
    /// Path to a debug binary for a specific target triple.
    ///
    /// ex: `$CARGO_TARGET_TMPDIR/cit/t0/foo/target/i686-apple-darwin/debug/foo`
    pub fn target_bin(&self, target: &str, b: &str) -> PathBuf {
        self.build_dir()
            .join(target)
            .join("debug")
            .join(&format!("{}{}", b, env::consts::EXE_SUFFIX))
    }
    /// Returns an iterator of paths within [`Project::root`] matching the glob pattern
    pub fn glob<P: AsRef<Path>>(&self, pattern: P) -> glob::Paths {
        let pattern = self.root().join(pattern);
        glob::glob(pattern.to_str().expect("failed to convert pattern to str"))
            .expect("failed to glob")
    }
    /// Overwrite a file with new content
    ///
    ///
    /// ```no_run
    /// # let p = cargo_test_support::project().build();
    /// p.change_file("src/lib.rs", "fn new_fn() {}");
    /// ```
    pub fn change_file(&self, path: impl AsRef<Path>, body: &str) {
        FileBuilder::new(self.root().join(path), body, false).mk()
    }
    /// Creates a `ProcessBuilder` to run a program in the project
    /// and wrap it in an Execs to assert on the execution.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use cargo_test_support::str;
    /// # let p = cargo_test_support::project().build();
    /// p.process(&p.bin("foo"))
    ///     .with_stdout_data(str!["bar\n"])
    ///     .run();
    /// ```
    pub fn process<T: AsRef<OsStr>>(&self, program: T) -> Execs {
        let mut p = process(program);
        p.cwd(self.root());
        execs().with_process_builder(p)
    }
    /// Safely run a process after `cargo build`.
    ///
    /// Windows has a problem where a process cannot be reliably
    /// be replaced, removed, or renamed immediately after executing it.
    /// The action may fail (with errors like Access is denied), or
    /// it may succeed, but future attempts to use the same filename
    /// will fail with "Already Exists".
    ///
    /// If you have a test that needs to do `cargo run` multiple
    /// times, you should instead use `cargo build` and use this
    /// method to run the executable. Each time you call this,
    /// use a new name for `dst`.
    /// See rust-lang/cargo#5481.
    pub fn rename_run(&self, src: &str, dst: &str) -> Execs {
        let src = self.bin(src);
        let dst = self.bin(dst);
        fs::rename(&src, &dst)
            .unwrap_or_else(|e| {
                panic!("Failed to rename `{:?}` to `{:?}`: {}", src, dst, e)
            });
        self.process(dst)
    }
    /// Returns the contents of `Cargo.lock`.
    pub fn read_lockfile(&self) -> String {
        self.read_file("Cargo.lock")
    }
    /// Returns the contents of a path in the project root
    pub fn read_file(&self, path: impl AsRef<Path>) -> String {
        let full = self.root().join(path);
        fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("could not read file {}: {}", full.display(), e))
    }
    /// Modifies `Cargo.toml` to remove all commented lines.
    pub fn uncomment_root_manifest(&self) {
        let contents = self.read_file("Cargo.toml").replace("#", "");
        fs::write(self.root().join("Cargo.toml"), contents).unwrap();
    }
    pub fn symlink(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) {
        let src = self.root().join(src.as_ref());
        let dst = self.root().join(dst.as_ref());
        #[cfg(unix)]
        {
            if let Err(e) = os::unix::fs::symlink(&src, &dst) {
                panic!("failed to symlink {:?} to {:?}: {:?}", src, dst, e);
            }
        }
        #[cfg(windows)]
        {
            if src.is_dir() {
                if let Err(e) = os::windows::fs::symlink_dir(&src, &dst) {
                    panic!("failed to symlink {:?} to {:?}: {:?}", src, dst, e);
                }
            } else {
                if let Err(e) = os::windows::fs::symlink_file(&src, &dst) {
                    panic!("failed to symlink {:?} to {:?}: {:?}", src, dst, e);
                }
            }
        }
    }
}
