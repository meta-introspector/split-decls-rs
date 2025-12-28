macro_rules! cae {
    () => {
        # [cfg (feature = "experimental")] # [doc = " Fully-committing Authenticated Encryption. __WARNING:__ Experimental feature."] pub mod cae ;
    };
}

cae!()