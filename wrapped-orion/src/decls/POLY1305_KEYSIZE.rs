macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! POLY1305_KEYSIZE {
    () => {
        deps!();
        # [doc = " The key size for Poly1305."] pub const POLY1305_KEYSIZE : usize = 32 ;
    };
}

POLY1305_KEYSIZE!();