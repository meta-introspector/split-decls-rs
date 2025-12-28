macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! IETF_CHACHA_NONCESIZE {
    () => {
        deps!();
        # [doc = " The nonce size for IETF ChaCha20."] pub const IETF_CHACHA_NONCESIZE : usize = 12 ;
    };
}

IETF_CHACHA_NONCESIZE!();