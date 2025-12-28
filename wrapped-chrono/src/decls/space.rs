macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! space {
    () => {
        deps!();
        # [doc = " Tries to consume one or more whitespace."] pub (super) fn space (s : & str) -> ParseResult < & str > { let s_ = s . trim_start () ; if s_ . len () < s . len () { Ok (s_) } else if s . is_empty () { Err (TOO_SHORT) } else { Err (INVALID) } }
    };
}

space!()