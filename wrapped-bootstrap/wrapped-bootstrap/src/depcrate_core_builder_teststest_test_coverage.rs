// Generated macro for test_test_coverage (function)
macro_rules! Depcrate_core_builder_teststest_test_coverage {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_test_coverage"}
// Dependencies: {}
# [test] fn test_test_coverage () { struct Case { cmd : & 'static [& 'static str] , expected : & 'static [& 'static str] , } let cases = & [Case { cmd : & ["test"] , expected : & ["coverage-map" , "coverage-run"] } , Case { cmd : & ["test" , "coverage"] , expected : & ["coverage-map" , "coverage-run"] } , Case { cmd : & ["test" , "coverage-map"] , expected : & ["coverage-map"] } , Case { cmd : & ["test" , "coverage-run"] , expected : & ["coverage-run"] } , Case { cmd : & ["test" , "coverage" , "--skip=coverage"] , expected : & [] } , Case { cmd : & ["test" , "coverage" , "--skip=tests/coverage"] , expected : & [] } , Case { cmd : & ["test" , "coverage" , "--skip=coverage-map"] , expected : & ["coverage-run"] } , Case { cmd : & ["test" , "coverage" , "--skip=coverage-run"] , expected : & ["coverage-map"] } , Case { cmd : & ["test" , "--skip=coverage-map" , "--skip=coverage-run"] , expected : & [] } , Case { cmd : & ["test" , "coverage" , "--skip=tests"] , expected : & [] } ,] ; for & Case { cmd , expected } in cases { println ! ("Testing case: {cmd:?}") ; let config = configure_with_args (cmd , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1]) ; let mut cache = run_build (& config . paths . clone () , config) ; let modes = cache . all :: < test :: Coverage > () . iter () . map (| (step , ()) | step . mode) . collect :: < Vec < _ > > () ; assert_eq ! (modes , expected) ; } }
};
}
