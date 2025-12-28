macro_rules! deps {
    () => {
        TestCase!();
    };
}

macro_rules! lex_err {
    () => {
        deps!();
        # [test] fn lex_err () { for case in TestCase :: list ("lexer/err") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let actual = lex (& case . text , infer_edition (& case . rs)) ; expect_file ! [case . rast] . assert_eq (& actual) } }
    };
}

lex_err!();