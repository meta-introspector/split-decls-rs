// Generated macro for make_test (function)
macro_rules! Depcratemake_test {
() => {
// Module: crate
// Provides: {"make_test"}
// Dependencies: {}
# [doc = " For a single test file, creates one or more test structures (one per revision) that can be"] # [doc = " handed over to the executor to run, possibly in parallel."] fn make_test (cx : & TestCollectorCx , collector : & mut TestCollector , testpaths : & TestPaths) { let test_path = if cx . config . mode == TestMode :: RunMake { testpaths . file . join ("rmake.rs") } else { testpaths . file . clone () } ; let early_props = EarlyProps :: from_file (& cx . config , & test_path) ; let revisions = if early_props . revisions . is_empty () || cx . config . mode == TestMode :: Incremental { vec ! [None] } else { early_props . revisions . iter () . map (| r | Some (r . as_str ())) . collect () } ; collector . tests . extend (revisions . into_iter () . map (| revision | { let src_file = fs :: File :: open (& test_path) . expect ("open test file to parse ignores") ; let test_name = make_test_name (& cx . config , testpaths , revision) ; let mut desc = make_test_description (& cx . config , & cx . cache , test_name , & test_path , src_file , revision , & mut collector . poisoned ,) ; if ! cx . config . force_rerun && is_up_to_date (cx , testpaths , & early_props , revision) { desc . ignore = true ; desc . ignore_message = Some ("up-to-date" . into ()) ; } let config = Arc :: clone (& cx . config) ; let testpaths = testpaths . clone () ; let revision = revision . map (str :: to_owned) ; CollectedTest { desc , config , testpaths , revision } })) ; }
};
}
