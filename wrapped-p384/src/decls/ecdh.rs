macro_rules! ecdh {
    () => {
        # [cfg (feature = "ecdh")] pub mod ecdh ;
    };
}

ecdh!();