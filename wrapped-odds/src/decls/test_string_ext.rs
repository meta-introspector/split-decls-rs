macro_rules! deps {
    () => {
        StringExt!();
    };
}

macro_rules! test_string_ext {
    () => {
        deps!();
        # [cfg (feature = "std-string")] # [test] fn test_string_ext () { let mut s = String :: new () ; let t = "αβγabc" ; StringExt :: insert_str (& mut s , 0 , t) ; assert_eq ! (s , t) ; StringExt :: insert_str (& mut s , 2 , "x") ; assert_eq ! (s , "αxβγabc") ; }
    };
}

test_string_ext!()