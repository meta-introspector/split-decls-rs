macro_rules! paserk {
    () => {
        # [cfg (feature = "paserk")] # [doc = " PASERK key-wrapping and serialization."] pub mod paserk ;
    };
}

paserk!();