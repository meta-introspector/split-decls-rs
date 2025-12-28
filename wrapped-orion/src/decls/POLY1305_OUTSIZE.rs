macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! POLY1305_OUTSIZE {
    () => {
        deps!();
        # [doc = " The output size for Poly1305."] pub const POLY1305_OUTSIZE : usize = 16 ;
    };
}

POLY1305_OUTSIZE!()