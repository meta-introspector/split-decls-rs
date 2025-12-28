macro_rules! BLAKE2B_MIN_KEY_SIZE {
    () => {
        # [doc = " The minimum `SecretKey` size (bytes) to be used by BLAKE2b in keyed mode."] const BLAKE2B_MIN_KEY_SIZE : usize = 32 ;
    };
}

BLAKE2B_MIN_KEY_SIZE!();