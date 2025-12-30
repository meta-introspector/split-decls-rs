// Generated macro for first (function)
macro_rules! Depcrate_core_builder_testsfirst {
() => {
// Module: crate::core::builder::tests
// Provides: {"first"}
// Dependencies: {}
fn first < A , B > (v : Vec < (A , B) >) -> Vec < A > { v . into_iter () . map (| (a , _) | a) . collect :: < Vec < _ > > () }
};
}
