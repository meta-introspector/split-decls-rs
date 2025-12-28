macro_rules! test_too_many_terminators {
    () => {
        # [test] fn test_too_many_terminators () { check_raw_str (r###"#"abc"##"### , Ok (1)) ; }
    };
}

test_too_many_terminators!();