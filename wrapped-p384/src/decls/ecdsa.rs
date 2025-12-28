macro_rules! ecdsa {
    () => {
        # [cfg (feature = "ecdsa-core")] pub mod ecdsa ;
    };
}

ecdsa!()