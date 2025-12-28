macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! AUTH_CTR {
    () => {
        deps!();
        # [doc = " The initial counter used for Poly1305 key generation."] pub (crate) const AUTH_CTR : u32 = 0 ;
    };
}

AUTH_CTR!()