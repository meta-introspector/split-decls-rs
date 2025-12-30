// Generated macro for run_single (function)
macro_rules! Depcraterun_single {
() => {
// Module: crate
// Provides: {"run_single"}
// Dependencies: {}
# [doc = " Runs the given runner, catching any panics and treating them as a failed test."] fn run_single (runner : Box < dyn FnOnce (bool) -> Outcome + Send > , test_mode : bool) -> Outcome { use std :: panic :: { catch_unwind , AssertUnwindSafe } ; catch_unwind (AssertUnwindSafe (move | | runner (test_mode))) . unwrap_or_else (| e | { let payload = e . downcast_ref :: < String > () . map (| s | s . as_str ()) . or (e . downcast_ref :: < & str > () . map (| s | * s)) ; let msg = match payload { Some (payload) => format ! ("test panicked: {payload}") , None => format ! ("test panicked") , } ; Outcome :: Failed (msg . into ()) }) }
};
}
