macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! POLY1305_BLOCKSIZE {
    () => {
        deps!();
        # [doc = " The blocksize which Poly1305 operates on."] const POLY1305_BLOCKSIZE : usize = 16 ;
    };
}

POLY1305_BLOCKSIZE!()