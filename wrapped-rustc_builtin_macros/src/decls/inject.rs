macro_rules! deps {
    () => {
        TestsNotSupport!();
    };
}

macro_rules! inject {
    () => {
        deps!();
        # [doc = " Traverse the crate, collecting all the test functions, eliding any"] # [doc = " existing main functions, and synthesizing a main test harness"] pub fn inject (krate : & mut ast :: Crate , sess : & Session , features : & Features , resolver : & mut dyn ResolverExpand ,) { let dcx = sess . dcx () ; let panic_strategy = sess . panic_strategy () ; let platform_panic_strategy = sess . target . panic_strategy ; let reexport_test_harness_main = attr :: first_attr_value_str_by_name (& krate . attrs , sym :: reexport_test_harness_main) ; let test_runner = get_test_runner (dcx , krate) ; if sess . is_test_crate () { let panic_strategy = match (panic_strategy , sess . opts . unstable_opts . panic_abort_tests) { (PanicStrategy :: Abort , true) => PanicStrategy :: Abort , (PanicStrategy :: Abort , false) => { if panic_strategy == platform_panic_strategy { } else { dcx . emit_err (errors :: TestsNotSupport { }) ; } PanicStrategy :: Unwind } (PanicStrategy :: Unwind , _) => PanicStrategy :: Unwind , } ; generate_test_harness (sess , resolver , reexport_test_harness_main , krate , features , panic_strategy , test_runner ,) } }
    };
}

inject!()