macro_rules! argon2i {
    () => {
        # [cfg (any (feature = "safe_api" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "safe_api" , feature = "alloc"))))] # [doc = " Argon2i password hashing function as described in the [P-H-C specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf)."] pub mod argon2i ;
    };
}

argon2i!();