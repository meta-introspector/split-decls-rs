macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        construct_tag ! { # [doc = " A type to represent the `Tag` that Poly1305 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 16 bytes."] (Tag , test_tag , POLY1305_OUTSIZE , POLY1305_OUTSIZE) }
    };
}

macro_259!()