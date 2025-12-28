macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! get_server_connect_timeout_in_milliseconds {
    () => {
        deps!();
        # [doc = " Get server connect timeout in milliseconds"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn get_server_connect_timeout_in_milliseconds () -> Result < libc :: c_int , Error > { crate :: init () ; let mut server_connect_timeout = 0 ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_SERVER_CONNECT_TIMEOUT as libc :: c_int , & mut server_connect_timeout)) ; Ok (server_connect_timeout) }
    };
}

get_server_connect_timeout_in_milliseconds!()