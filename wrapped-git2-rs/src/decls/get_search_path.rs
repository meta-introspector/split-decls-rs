macro_rules! deps {
    () => {
        Buf!();
        ConfigLevel!();
        Error!();
    };
}

macro_rules! get_search_path {
    () => {
        deps!();
        # [doc = " Get the search path for a given level of config data."] # [doc = ""] # [doc = " `level` must be one of [`ConfigLevel::System`], [`ConfigLevel::Global`],"] # [doc = " [`ConfigLevel::XDG`], [`ConfigLevel::ProgramData`]."] # [doc = ""] # [doc = " This function is unsafe as it mutates the global state but cannot guarantee"] # [doc = " thread-safety. It needs to be externally synchronized with calls to access"] # [doc = " the global state."] pub unsafe fn get_search_path (level : ConfigLevel) -> Result < CString , Error > { crate :: init () ; let buf = Buf :: new () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_SEARCH_PATH as libc :: c_int , level as libc :: c_int , buf . raw () as * const _)) ; buf . into_c_string () }
    };
}

get_search_path!();