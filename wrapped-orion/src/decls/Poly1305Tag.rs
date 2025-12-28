macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! Poly1305Tag {
    () => {
        deps!();
        # [doc = " Type for a Poly1305 tag."] type Poly1305Tag = [u8 ; POLY1305_OUTSIZE] ;
    };
}

Poly1305Tag!()