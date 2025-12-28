macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! RawToken {
    () => {
        deps!();
        # [doc = " A combination of a parsed [`Token`] as well as the range of bytes that were consumed to parse it."] struct RawToken < 'a > { # [doc = " The parsed token."] token : Token < 'a > , token_range : Range < usize > , }
    };
}

RawToken!();