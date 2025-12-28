macro_rules! deps {
    () => {
        RawStrError!();
    };
}

macro_rules! test_unterminated {
    () => {
        deps!();
        # [test] fn test_unterminated () { check_raw_str (r#"#"abc"# , Err (RawStrError :: NoTerminator { expected : 1 , found : 0 , possible_terminator_offset : None }) ,) ; check_raw_str (r###"##"abc"#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 1 , possible_terminator_offset : Some (7) , }) ,) ; check_raw_str (r###"##"abc#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 0 , possible_terminator_offset : None }) ,) }
    };
}

test_unterminated!();