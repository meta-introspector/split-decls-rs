macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! set_mwindow_size {
    () => {
        deps!();
        # [doc = " Set the maximum mmap window size"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_mwindow_size (size : libc :: size_t) -> Result < () , Error > { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_MWINDOW_SIZE as libc :: c_int , size)) ; Ok (()) }
    };
}

set_mwindow_size!()