macro_rules! ed25519 {
    () => {
        # [cfg (not (feature = "disable-signatures"))] mod ed25519 ;
    };
}

ed25519!();