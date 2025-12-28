macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! test_unescape_str_warn {
    () => {
        deps!();
        # [test] fn test_unescape_str_warn () { fn check (literal : & str , expected : & [(Range < usize > , Result < char , EscapeError >)]) { let mut unescaped = Vec :: with_capacity (literal . len ()) ; unescape_str (literal , | range , res | unescaped . push ((range , res))) ; assert_eq ! (unescaped , expected) ; } check ("\\\n" , & []) ; check ("\\\n " , & []) ; check ("\\\n \u{a0} x" , & [(0 .. 5 , Err (EscapeError :: UnskippedWhitespaceWarning)) , (3 .. 5 , Ok ('\u{a0}')) , (5 .. 6 , Ok (' ')) , (6 .. 7 , Ok ('x')) ,] ,) ; check ("\\\n  \n  x" , & [(0 .. 7 , Err (EscapeError :: MultipleSkippedLinesWarning)) , (7 .. 8 , Ok ('x')) ,] ,) ; }
    };
}

test_unescape_str_warn!()