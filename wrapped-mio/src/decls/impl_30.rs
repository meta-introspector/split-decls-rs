macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (all (unix , not (mio_unsupported_force_poll_poll) , not (any (target_os = "aix" , target_os = "espidf" , target_os = "haiku" , target_os = "fuchsia" , target_os = "hermit" , target_os = "hurd" , target_os = "nto" , target_os = "solaris" , target_os = "vita" , target_os = "cygwin" ,)) ,))] impl AsRawFd for Registry { fn as_raw_fd (& self) -> RawFd { self . selector . as_raw_fd () } }
    };
}

impl_30!()