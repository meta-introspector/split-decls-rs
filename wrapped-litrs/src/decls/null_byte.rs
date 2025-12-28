macro_rules! deps {
    () => {
        CStringLit!();
    };
}

macro_rules! null_byte {
    () => {
        deps!();
        # [test] fn null_byte () { assert_err ! (CStringLit , r#"c"\x00""# , DisallowedNulEscape , 2 .. 6) ; assert_err ! (CStringLit , r#"c"\u{0}""# , DisallowedNulEscape , 2 .. 7) ; assert_err ! (CStringLit , r#"c"\u{00}""# , DisallowedNulEscape , 2 .. 8) ; assert_err ! (CStringLit , r#"c"\u{000}""# , DisallowedNulEscape , 2 .. 9) ; assert_err ! (CStringLit , r#"c"\u{0000}""# , DisallowedNulEscape , 2 .. 10) ; assert_err ! (CStringLit , r#"c"\u{00000}""# , DisallowedNulEscape , 2 .. 11) ; assert_err ! (CStringLit , r#"c"\u{000000}""# , DisallowedNulEscape , 2 .. 12) ; assert_err ! (CStringLit , r#"c" \u{00}""# , DisallowedNulEscape , 3 .. 9) ; assert_err ! (CStringLit , r#"c"\u{0}🦊""# , DisallowedNulEscape , 2 .. 7) ; assert_err ! (CStringLit , r#"c"лиса\u{0__}""# , DisallowedNulEscape , 10 .. 17) ; }
    };
}

null_byte!()