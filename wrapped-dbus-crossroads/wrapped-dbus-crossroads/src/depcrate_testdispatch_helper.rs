// Generated macro for dispatch_helper (function)
macro_rules! Depcrate_testdispatch_helper {
() => {
// Module: crate::test
// Provides: {"dispatch_helper"}
// Dependencies: {}
fn dispatch_helper (cr : & mut Crossroads , msg : Message) -> Message { let mut r = dispatch_helper2 (cr , msg) ; assert_eq ! (r . len () , 1) ; r [0] . as_result () . unwrap () ; r . into_iter () . next () . unwrap () }
};
}
