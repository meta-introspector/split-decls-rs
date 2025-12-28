macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! CHACHA_BLOCKSIZE {
    () => {
        deps!();
        # [doc = " The blocksize which ChaCha20 operates on."] pub (crate) const CHACHA_BLOCKSIZE : usize = 64 ;
    };
}

CHACHA_BLOCKSIZE!()