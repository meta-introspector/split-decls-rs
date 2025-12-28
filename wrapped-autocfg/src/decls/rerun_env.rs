macro_rules! rerun_env {
    () => {
        # [doc = " Writes a line telling Cargo to rerun the build script if the environment"] # [doc = " variable `var` changes."] # [doc = ""] # [doc = " This looks like: `cargo:rerun-if-env-changed=VAR`"] # [doc = ""] # [doc = " This requires at least cargo 0.21.0, corresponding to rustc 1.20.0.  Earlier"] # [doc = " versions of cargo will simply ignore the directive."] pub fn rerun_env (var : & str) { println ! ("cargo:rerun-if-env-changed={}" , var) ; }
    };
}

rerun_env!()