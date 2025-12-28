macro_rules! deps {
    () => {
        TestCtxt!();
        TestHarnessGenerator!();
        EntryPointCleaner!();
        Path!();
    };
}

macro_rules! generate_test_harness {
    () => {
        deps!();
        # [doc = " Crawl over the crate, inserting test reexports and the test main function"] fn generate_test_harness (sess : & Session , resolver : & mut dyn ResolverExpand , reexport_test_harness_main : Option < Symbol > , krate : & mut ast :: Crate , features : & Features , panic_strategy : PanicStrategy , test_runner : Option < ast :: Path > ,) { let econfig = ExpansionConfig :: default (sym :: test , features) ; let ext_cx = ExtCtxt :: new (sess , econfig , resolver , None) ; let expn_id = ext_cx . resolver . expansion_for_ast_pass (DUMMY_SP , AstPass :: TestHarness , & [sym :: test , sym :: rustc_attrs , sym :: coverage_attribute] , None ,) ; let def_site = DUMMY_SP . with_def_site_ctxt (expn_id . to_expn_id ()) ; let mut cleaner = EntryPointCleaner { sess , depth : 0 , def_site } ; cleaner . visit_crate (krate) ; let cx = TestCtxt { ext_cx , panic_strategy , def_site , test_cases : Vec :: new () , reexport_test_harness_main , test_runner , } ; TestHarnessGenerator { cx , tests : Vec :: new () } . visit_crate (krate) ; }
    };
}

generate_test_harness!();