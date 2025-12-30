// Generated macro for runner (function)
macro_rules! Depcraterunner {
() => {
// Module: crate
// Provides: {"runner"}
// Dependencies: {}
# [doc = " Custom-test-framework runner. Should not be called directly."] # [doc (hidden)] pub fn runner (benches : & [& dyn Fn ()]) { for bench in benches { bench () ; } Criterion :: default () . configure_from_args () . final_summary () ; }
};
}
