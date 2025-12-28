macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
        ConfigLevel!();
    };
}

macro_rules! set_search_path {
    () => {
        deps!();
        # [doc = " Set the search path for a level of config data. The search path applied to"] # [doc = " shared attributes and ignore files, too."] # [doc = ""] # [doc = " `level` must be one of [`ConfigLevel::System`], [`ConfigLevel::Global`],"] # [doc = " [`ConfigLevel::XDG`], [`ConfigLevel::ProgramData`]."] # [doc = ""] # [doc = " `path` lists directories delimited by `GIT_PATH_LIST_SEPARATOR`."] # [doc = " Use magic path `$PATH` to include the old value of the path"] # [doc = " (if you want to prepend or append, for instance)."] # [doc = ""] # [doc = " This function is unsafe as it mutates the global state but cannot guarantee"] # [doc = " thread-safety. It needs to be externally synchronized with calls to access"] # [doc = " the global state."] pub unsafe fn set_search_path < P > (level : ConfigLevel , path : P) -> Result < () , Error > where P : IntoCString , { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SEARCH_PATH as libc :: c_int , level as libc :: c_int , path . into_c_string () ?. as_ptr ())) ; Ok (()) }
    };
}

set_search_path!();