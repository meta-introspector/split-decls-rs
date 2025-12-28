macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        # [cfg (not (any (windows , target_os = "unknown")))] impl < Stream : AsFd > IsTerminal for Stream { # [inline] fn is_terminal (& self) -> bool { # [cfg (any (unix , target_os = "wasi"))] { let fd = self . as_fd () ; unsafe { libc :: isatty (fd . as_raw_fd ()) != 0 } } # [cfg (target_os = "hermit")] { use std :: os :: hermit :: io :: AsRawFd ; hermit_abi :: isatty (self . as_fd () . as_fd () . as_raw_fd ()) } } }
    };
}

impl_2!();