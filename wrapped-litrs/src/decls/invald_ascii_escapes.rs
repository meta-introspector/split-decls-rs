macro_rules! deps {
    () => {
        StringLit!();
    };
}

macro_rules! invald_ascii_escapes {
    () => {
        deps!();
        # [test] fn invald_ascii_escapes () { assert_err ! (StringLit , r#""\x80""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#""🦊\x81""# , NonAsciiXEscape , 5 .. 9) ; assert_err ! (StringLit , r#"" \x8a""# , NonAsciiXEscape , 2 .. 6) ; assert_err ! (StringLit , r#""\x8Ff""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#""\xa0 ""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#""నక్క\xB0""# , NonAsciiXEscape , 13 .. 17) ; assert_err ! (StringLit , r#""\xc3నక్క""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#""\xDf🦊""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#""నక్క\xffనక్క""# , NonAsciiXEscape , 13 .. 17) ; assert_err ! (StringLit , r#""\xfF ""# , NonAsciiXEscape , 1 .. 5) ; assert_err ! (StringLit , r#"" \xFf""# , NonAsciiXEscape , 2 .. 6) ; assert_err ! (StringLit , r#""నక్క  \xFF""# , NonAsciiXEscape , 15 .. 19) ; }
    };
}

invald_ascii_escapes!();