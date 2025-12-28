macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! set_mwindow_file_limit {
    () => {
        deps!();
        # [doc = " Set the maximum number of files that can be mapped at any time"] # [doc = " by the library. The default (0) is unlimited."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_mwindow_file_limit (limit : libc :: size_t) -> Result < () , Error > { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_MWINDOW_FILE_LIMIT as libc :: c_int , limit)) ; Ok (()) }
    };
}

set_mwindow_file_limit!();