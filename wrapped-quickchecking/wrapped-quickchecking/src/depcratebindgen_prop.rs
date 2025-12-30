// Generated macro for bindgen_prop (function)
macro_rules! Depcratebindgen_prop {
() => {
// Module: crate
// Provides: {"bindgen_prop"}
// Dependencies: {}
# [allow (clippy :: needless_pass_by_value)] fn bindgen_prop (header : fuzzers :: HeaderC) -> TestResult { match run_predicate_script (& header) { Ok (o) => TestResult :: from_bool (o . status . success ()) , Err (e) => { println ! ("{e:?}") ; TestResult :: from_bool (false) } } }
};
}
