macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! poly1305 {
    () => {
        deps!();
        # [doc = " Poly1305 as specified in the [RFC 8439](https://tools.ietf.org/html/rfc8439)."] pub mod poly1305 ;
    };
}

poly1305!();