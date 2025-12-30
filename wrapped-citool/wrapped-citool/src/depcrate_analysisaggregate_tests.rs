// Generated macro for aggregate_tests (function)
macro_rules! Depcrate_analysisaggregate_tests {
() => {
// Module: crate::analysis
// Provides: {"aggregate_tests"}
// Dependencies: {}
# [doc = " Extracts all tests from the passed metrics and map them to their outcomes."] fn aggregate_tests (metrics : & JsonRoot) -> TestSuiteData { let mut tests = HashMap :: new () ; let test_suites = get_test_suites (& metrics) ; for suite in test_suites { let stage = match suite . metadata { TestSuiteMetadata :: CargoPackage { stage , .. } => stage , TestSuiteMetadata :: Compiletest { stage , .. } => stage , } as u8 ; for test in & suite . tests { let is_doctest = matches ! (suite . metadata , TestSuiteMetadata :: CargoPackage { .. }) && test . name . contains ("(line") ; let test_entry = Test { name : utils :: normalize_path_delimiters (& test . name) . to_string () , stage , is_doctest , } ; tests . insert (test_entry , test . outcome . clone ()) ; } } TestSuiteData { tests } }
};
}
