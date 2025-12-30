// Generated macro for run_test_inner (function)
macro_rules! Depcrate_executorrun_test_inner {
() => {
// Module: crate::executor
// Provides: {"run_test_inner"}
// Dependencies: {}
# [doc = " Runs a single test, within the dedicated thread spawned by the caller."] fn run_test_inner (id : TestId , should_panic : ShouldPanic , runnable_test : RunnableTest , completion_sender : mpsc :: Sender < TestCompletion > ,) { let capture = CaptureKind :: for_config (& runnable_test . config) ; if capture . should_set_panic_hook () { panic_hook :: set_capture_buf (Default :: default ()) ; } if let CaptureKind :: Old { ref buf } = capture { io :: set_output_capture (Some (Arc :: clone (buf))) ; } let stdout = capture . stdout () ; let stderr = capture . stderr () ; let panic_payload = panic :: catch_unwind (move | | runnable_test . run (stdout , stderr)) . err () ; if let Some (panic_buf) = panic_hook :: take_capture_buf () { let panic_buf = panic_buf . lock () . unwrap_or_else (| e | e . into_inner ()) ; write ! (stderr , "{panic_buf}") ; } if matches ! (capture , CaptureKind :: Old { .. }) { io :: set_output_capture (None) ; } let outcome = match (should_panic , panic_payload) { (ShouldPanic :: No , None) | (ShouldPanic :: Yes , Some (_)) => TestOutcome :: Succeeded , (ShouldPanic :: No , Some (_)) => TestOutcome :: Failed { message : None } , (ShouldPanic :: Yes , None) => { TestOutcome :: Failed { message : Some ("test did not panic as expected") } } } ; let stdout = capture . into_inner () ; completion_sender . send (TestCompletion { id , outcome , stdout }) . unwrap () ; }
};
}
