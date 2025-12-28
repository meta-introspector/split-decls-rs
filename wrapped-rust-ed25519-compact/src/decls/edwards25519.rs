macro_rules! edwards25519 {
    () => {
        # [cfg (not (feature = "disable-signatures"))] mod edwards25519 ;
    };
}

edwards25519!()