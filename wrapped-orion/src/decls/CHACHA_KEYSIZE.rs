macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! CHACHA_KEYSIZE {
    () => {
        deps!();
        # [doc = " The key size for ChaCha20."] pub const CHACHA_KEYSIZE : usize = 32 ;
    };
}

CHACHA_KEYSIZE!();