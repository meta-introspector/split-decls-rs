macro_rules! Dur {
    () => {
        # [cfg (all (feature = "server" , feature = "http1"))] # [derive (Clone , Copy , Debug)] pub (crate) enum Dur { Default (Option < Duration >) , Configured (Option < Duration >) , }
    };
}

Dur!();