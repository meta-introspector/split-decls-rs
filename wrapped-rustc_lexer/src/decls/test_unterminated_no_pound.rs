macro_rules! deps {
    () => {
        RawStrError!();
    };
}

macro_rules! test_unterminated_no_pound {
    () => {
        deps!();
        # [test] fn test_unterminated_no_pound () { check_raw_str (r#"""# , Err (RawStrError :: NoTerminator { expected : 0 , found : 0 , possible_terminator_offset : None }) ,) ; }
    };
}

test_unterminated_no_pound!();