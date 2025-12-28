macro_rules! x25519 {
    () => {
        # [cfg (feature = "x25519")] pub mod x25519 ;
    };
}

x25519!()