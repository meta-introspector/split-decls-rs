macro_rules! deps {
    () => {
        RawStrError!();
    };
}

macro_rules! test_invalid_start {
    () => {
        deps!();
        # [test] fn test_invalid_start () { check_raw_str (r##"#~"abc"#"## , Err (RawStrError :: InvalidStarter { bad_char : '~' })) ; }
    };
}

test_invalid_start!()