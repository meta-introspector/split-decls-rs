macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! colon_or_space {
    () => {
        deps!();
        # [doc = " Consumes any number (including zero) of colon or spaces."] pub (crate) fn colon_or_space (s : & str) -> ParseResult < & str > { Ok (s . trim_start_matches (| c : char | c == ':' || c . is_whitespace ())) }
    };
}

colon_or_space!()