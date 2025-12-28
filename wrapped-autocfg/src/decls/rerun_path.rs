macro_rules! rerun_path {
    () => {
        # [doc = " Writes a line telling Cargo to rerun the build script if `path` changes."] # [doc = ""] # [doc = " This looks like: `cargo:rerun-if-changed=PATH`"] # [doc = ""] # [doc = " This requires at least cargo 0.7.0, corresponding to rustc 1.6.0.  Earlier"] # [doc = " versions of cargo will simply ignore the directive."] pub fn rerun_path (path : & str) { println ! ("cargo:rerun-if-changed={}" , path) ; }
    };
}

rerun_path!();