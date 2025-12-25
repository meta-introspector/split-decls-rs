use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Establish a process's test environment
pub trait TestEnvCommandExt: Sized {
    fn test_env(mut self) -> Self {
        for (k, _v) in env::vars() {
            if k.starts_with("CARGO_") {
                self = self.env_remove(&k);
            }
        }
        if env::var_os("RUSTUP_TOOLCHAIN").is_some() {
            static RUSTC_DIR: OnceLock<PathBuf> = OnceLock::new();
            let rustc_dir = RUSTC_DIR.get_or_init(|| {
                match ProcessBuilder::new("rustup")
                    .args(&["which", "rustc"])
                    .exec_with_output()
                {
                    Ok(output) => {
                        let s = std::str::from_utf8(&output.stdout).expect("utf8").trim();
                        let mut p = PathBuf::from(s);
                        p.pop();
                        p
                    }
                    Err(e) => {
                        panic!("RUSTUP_TOOLCHAIN was set, but could not run rustup: {}", e);
                    }
                }
            });
            let path = env::var_os("PATH").unwrap_or_default();
            let paths = env::split_paths(&path);
            let new_path =
                env::join_paths(std::iter::once(rustc_dir.clone()).chain(paths)).unwrap();
            self = self.env("PATH", new_path);
        }
        self = self
            .current_dir(&paths::root())
            .env("HOME", paths::home())
            .env("CARGO_HOME", paths::cargo_home())
            .env("__CARGO_TEST_ROOT", paths::global_root())
            .env("__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS", "stable")
            .env("__CARGO_TEST_DISABLE_GLOBAL_KNOWN_HOST", "1")
            .env("__CARGO_TEST_FIXED_RETRY_SLEEP_MS", "1")
            .env("CARGO_INCREMENTAL", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env_remove("__CARGO_DEFAULT_LIB_METADATA")
            .env_remove("ALL_PROXY")
            .env_remove("EMAIL")
            .env_remove("GIT_AUTHOR_EMAIL")
            .env_remove("GIT_AUTHOR_NAME")
            .env_remove("GIT_COMMITTER_EMAIL")
            .env_remove("GIT_COMMITTER_NAME")
            .env_remove("http_proxy")
            .env_remove("HTTPS_PROXY")
            .env_remove("https_proxy")
            .env_remove("MAKEFLAGS")
            .env_remove("MFLAGS")
            .env_remove("MSYSTEM")
            .env_remove("RUSTC")
            .env_remove("RUST_BACKTRACE")
            .env_remove("RUSTC_WORKSPACE_WRAPPER")
            .env_remove("RUSTC_WRAPPER")
            .env_remove("RUSTDOC")
            .env_remove("RUSTDOCFLAGS")
            .env_remove("RUSTFLAGS")
            .env_remove("SSH_AUTH_SOCK")
            .env_remove("USER")
            .env_remove("XDG_CONFIG_HOME")
            .env_remove("OUT_DIR");
        if cfg!(windows) {
            self = self.env("USERPROFILE", paths::home());
        }
        self
    }
    fn current_dir<S: AsRef<std::path::Path>>(self, path: S) -> Self;
    fn env<S: AsRef<std::ffi::OsStr>>(self, key: &str, value: S) -> Self;
    fn env_remove(self, key: &str) -> Self;
}
