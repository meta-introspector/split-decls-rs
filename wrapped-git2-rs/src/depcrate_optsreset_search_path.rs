// Generated macro for reset_search_path (function)
macro_rules! Depcrate_optsreset_search_path {
() => {
// Module: crate::opts
// Provides: {"reset_search_path"}
// Dependencies: {}
# [doc = " Reset the search path for a given level of config data to the default"] # [doc = " (generally based on environment variables)."] # [doc = ""] # [doc = " `level` must be one of [`ConfigLevel::System`], [`ConfigLevel::Global`],"] # [doc = " [`ConfigLevel::XDG`], [`ConfigLevel::ProgramData`]."] # [doc = ""] # [doc = " This function is unsafe as it mutates the global state but cannot guarantee"] # [doc = " thread-safety. It needs to be externally synchronized with calls to access"] # [doc = " the global state."] pub unsafe fn reset_search_path (level : ConfigLevel) -> Result < () , Error > { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SEARCH_PATH as libc :: c_int , level as libc :: c_int , core :: ptr :: null ::< u8 > ())) ; Ok (()) }
};
}
