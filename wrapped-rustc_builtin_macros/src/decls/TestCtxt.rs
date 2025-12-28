macro_rules! deps {
    () => {
        Test!();
        Path!();
    };
}

macro_rules! TestCtxt {
    () => {
        deps!();
        struct TestCtxt < 'a > { ext_cx : ExtCtxt < 'a > , panic_strategy : PanicStrategy , def_site : Span , test_cases : Vec < Test > , reexport_test_harness_main : Option < Symbol > , test_runner : Option < ast :: Path > , }
    };
}

TestCtxt!();