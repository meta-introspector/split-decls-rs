// Generated macro for TestCtxt (struct)
macro_rules! Depcrate_test_harnessTestCtxt {
() => {
// Module: crate::test_harness
// Provides: {"TestCtxt"}
// Dependencies: {}
struct TestCtxt < 'a > { ext_cx : ExtCtxt < 'a > , panic_strategy : PanicStrategy , def_site : Span , test_cases : Vec < Test > , reexport_test_harness_main : Option < Symbol > , test_runner : Option < ast :: Path > , }
};
}
