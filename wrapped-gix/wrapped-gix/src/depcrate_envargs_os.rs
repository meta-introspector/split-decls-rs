// Generated macro for args_os (function)
macro_rules! Depcrate_envargs_os {
() => {
// Module: crate::env
// Provides: {"args_os"}
// Dependencies: {}
# [doc = " Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms."] # [doc = " It does not change the input arguments on any other platform."] # [doc = ""] # [doc = " Note that this ignores `core.precomposeUnicode` as git-config isn't available yet. It's default enabled in modern git though,"] # [doc = " and generally decomposed unicode is nothing one would want in a git repository."] pub fn args_os () -> impl Iterator < Item = OsString > { args_os_opt (cfg ! (target_vendor = "apple")) }
};
}
