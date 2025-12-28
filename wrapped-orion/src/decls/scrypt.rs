macro_rules! scrypt {
    () => {
        # [cfg (any (feature = "safe_api" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "safe_api" , feature = "alloc"))))] # [doc = " Scrypt Password-Based Key Derivation Function as specified in the [RFC 7914](https://datatracker.ietf.org/doc/html/rfc7914.html)."] pub mod scrypt ;
    };
}

scrypt!();