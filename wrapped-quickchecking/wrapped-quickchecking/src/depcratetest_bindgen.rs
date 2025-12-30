// Generated macro for test_bindgen (function)
macro_rules! Depcratetest_bindgen {
() => {
// Module: crate
// Provides: {"test_bindgen"}
// Dependencies: {}
# [doc = " Instantiate a Quickcheck object and use it to run property tests using"] # [doc = " fuzzed C headers generated with types defined in the `fuzzers` module."] # [doc = " Success/Failure is dictated by the result of passing the fuzzed headers"] # [doc = " to the `csmith-fuzzing/predicate.py` script."] pub fn test_bindgen (generate_range : usize , tests : u64 , output_path : Option < & Path > ,) { if let Some (path) = output_path { CONTEXT . lock () . unwrap () . output_path = Some (path . display () . to_string ()) ; } QuickCheck :: new () . tests (tests) . gen (Gen :: new (generate_range)) . quickcheck (bindgen_prop as fn (fuzzers :: HeaderC) -> TestResult) ; }
};
}
