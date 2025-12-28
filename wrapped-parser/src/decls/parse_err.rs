macro_rules! deps {
    () => {
        TestCase!();
        TopEntryPoint!();
    };
}

macro_rules! parse_err {
    () => {
        deps!();
        # [test] fn parse_err () { for case in TestCase :: list ("parser/err") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let (actual , errors) = parse (TopEntryPoint :: SourceFile , & case . text , Edition :: CURRENT) ; assert ! (errors , "no errors in an ERR file {}:\n{actual}" , case . rs . display ()) ; expect_file ! [case . rast] . assert_eq (& actual) } }
    };
}

parse_err!()