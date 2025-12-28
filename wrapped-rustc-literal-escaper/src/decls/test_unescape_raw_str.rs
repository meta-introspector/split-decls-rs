macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! test_unescape_raw_str {
    () => {
        deps!();
        # [test] fn test_unescape_raw_str () { fn check (literal : & str , expected : & [(Range < usize > , Result < char , EscapeError >)]) { let mut unescaped = Vec :: with_capacity (literal . len ()) ; check_raw_str (literal , | range , res | unescaped . push ((range , res))) ; assert_eq ! (unescaped , expected) ; } check ("\r" , & [(0 .. 1 , Err (EscapeError :: BareCarriageReturnInRawString))] ,) ; check ("\rx" , & [(0 .. 1 , Err (EscapeError :: BareCarriageReturnInRawString)) , (1 .. 2 , Ok ('x')) ,] ,) ; }
    };
}

test_unescape_raw_str!();