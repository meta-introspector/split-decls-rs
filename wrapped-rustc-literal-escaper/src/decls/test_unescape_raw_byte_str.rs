macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! test_unescape_raw_byte_str {
    () => {
        deps!();
        # [test] fn test_unescape_raw_byte_str () { fn check (literal : & str , expected : & [(Range < usize > , Result < u8 , EscapeError >)]) { let mut unescaped = Vec :: with_capacity (literal . len ()) ; check_raw_byte_str (literal , | range , res | unescaped . push ((range , res))) ; assert_eq ! (unescaped , expected) ; } check ("\r" , & [(0 .. 1 , Err (EscapeError :: BareCarriageReturnInRawString))] ,) ; check ("🦀" , & [(0 .. 4 , Err (EscapeError :: NonAsciiCharInByte))]) ; check ("🦀a" , & [(0 .. 4 , Err (EscapeError :: NonAsciiCharInByte)) , (4 .. 5 , Ok (b'a')) ,] ,) ; }
    };
}

test_unescape_raw_byte_str!()