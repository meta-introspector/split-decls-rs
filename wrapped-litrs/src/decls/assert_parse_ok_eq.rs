macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! assert_parse_ok_eq {
    () => {
        deps!();
        # [track_caller] pub (crate) fn assert_parse_ok_eq < T : PartialEq + Debug + Display > (input : & str , result : Result < T , ParseError > , expected : T , parse_method : & str ,) { match result { Ok (actual) if actual == expected => { if actual . to_string () != input { panic ! ("formatting does not yield original input `{input}`: {actual:?}") ; } } Ok (actual) => { panic ! ("unexpected parsing result (with `{parse_method}`) for `{input}`:\n\
                actual:    {actual:?}\n\
                expected:  {expected:?}") ; } Err (e) => { panic ! ("expected `{input}` to be parsed (with `{parse_method}`) successfully, \
                but it failed: {e:?}") ; } } }
    };
}

assert_parse_ok_eq!();