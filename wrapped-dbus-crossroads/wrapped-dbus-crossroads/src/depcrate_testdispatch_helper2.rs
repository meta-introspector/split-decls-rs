// Generated macro for dispatch_helper2 (function)
macro_rules! Depcrate_testdispatch_helper2 {
() => {
// Module: crate::test
// Provides: {"dispatch_helper2"}
// Dependencies: {}
fn dispatch_helper2 (cr : & mut Crossroads , mut msg : Message) -> Vec < Message > { msg . set_serial (57) ; let r = RefCell :: new (vec ! ()) ; cr . handle_message (msg , & r) . unwrap () ; r . into_inner () }
};
}
