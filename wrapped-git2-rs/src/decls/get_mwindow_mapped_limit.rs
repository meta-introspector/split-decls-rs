macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! get_mwindow_mapped_limit {
    () => {
        deps!();
        # [doc = " Get the maximum memory that will be mapped in total by the library"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is reading a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn get_mwindow_mapped_limit () -> Result < libc :: size_t , Error > { crate :: init () ; let mut limit = 0 ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_MWINDOW_MAPPED_LIMIT as libc :: c_int , & mut limit)) ; Ok (limit) }
    };
}

get_mwindow_mapped_limit!();