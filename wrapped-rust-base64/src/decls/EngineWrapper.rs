macro_rules! deps {
    () => {
        Engine!();
        Alphabet!();
        DecodePaddingMode!();
    };
}

macro_rules! EngineWrapper {
    () => {
        deps!();
        # [doc = " A wrapper to make using engines in rstest fixtures easier."] # [doc = " The functions don't need to be instance methods, but rstest does seem"] # [doc = " to want an instance, so instances are passed to test functions and then ignored."] trait EngineWrapper { type Engine : Engine ; # [doc = " Return an engine configured for RFC standard base64"] fn standard () -> Self :: Engine ; # [doc = " Return an engine configured for RFC standard base64, except with no padding appended on"] # [doc = " encode, and required no padding on decode."] fn standard_unpadded () -> Self :: Engine ; # [doc = " Return an engine configured for RFC standard alphabet with the provided encode and decode"] # [doc = " pad settings"] fn standard_with_pad_mode (encode_pad : bool , decode_pad_mode : DecodePaddingMode) -> Self :: Engine ; # [doc = " Return an engine configured for RFC standard base64 that allows invalid trailing bits"] fn standard_allow_trailing_bits () -> Self :: Engine ; # [doc = " Return an engine configured with a randomized alphabet and config"] fn random < R : rand :: Rng > (rng : & mut R) -> Self :: Engine ; # [doc = " Return an engine configured with the specified alphabet and randomized config"] fn random_alphabet < R : rand :: Rng > (rng : & mut R , alphabet : & Alphabet) -> Self :: Engine ; }
    };
}

EngineWrapper!()