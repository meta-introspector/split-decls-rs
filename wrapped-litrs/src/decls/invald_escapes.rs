macro_rules! deps {
    () => {
        ByteLit!();
    };
}

macro_rules! invald_escapes {
    () => {
        deps!();
        # [test] fn invald_escapes () { assert_err ! (ByteLit , r"b'\a'" , UnknownEscape , 2 .. 4) ; assert_err ! (ByteLit , r"b'\y'" , UnknownEscape , 2 .. 4) ; assert_err ! (ByteLit , r"b'\" , UnterminatedEscape , 2 .. 3) ; assert_err ! (ByteLit , r"b'\x'" , UnterminatedEscape , 2 .. 5) ; assert_err ! (ByteLit , r"b'\x1'" , InvalidXEscape , 2 .. 6) ; assert_err ! (ByteLit , r"b'\xaj'" , InvalidXEscape , 2 .. 6) ; assert_err ! (ByteLit , r"b'\xjb'" , InvalidXEscape , 2 .. 6) ; }
    };
}

invald_escapes!()