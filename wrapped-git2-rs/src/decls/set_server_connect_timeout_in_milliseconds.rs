macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! set_server_connect_timeout_in_milliseconds {
    () => {
        deps!();
        # [doc = " Set server connect timeout in milliseconds"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_server_connect_timeout_in_milliseconds (timeout : libc :: c_int ,) -> Result < () , Error > { crate :: init () ; let error = raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SERVER_CONNECT_TIMEOUT as libc :: c_int , timeout ,) ; debug_assert ! (error >= 0) ; Ok (()) }
    };
}

set_server_connect_timeout_in_milliseconds!();