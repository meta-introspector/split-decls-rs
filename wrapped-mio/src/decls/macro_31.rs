macro_rules! deps {
    () => {
        Poll!();
    };
}

macro_rules! macro_31 {
    () => {
        deps!();
        cfg_os_poll ! { # [cfg (all (unix , not (mio_unsupported_force_poll_poll) , not (any (target_os = "aix" , target_os = "espidf" , target_os = "hermit" , target_os = "hurd" , target_os = "nto" , target_os = "solaris" , target_os = "vita" , target_os = "cygwin" ,)) ,))] # [test] pub fn as_raw_fd () { let poll = Poll :: new () . unwrap () ; assert ! (poll . as_raw_fd () > 0) ; } }
    };
}

macro_31!()