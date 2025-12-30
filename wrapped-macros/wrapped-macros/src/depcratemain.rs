// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let user = "Parker" ; let msg = format_compact ! ("Hello {}" , user) ; println ! ("CompactString: {}" , msg) ; assert ! (! msg . is_heap_allocated ()) ; let msg_std = format ! ("Hello {}" , user) ; assert_eq ! (msg , msg_std) ; println ! ("std::String: {}" , msg) ; }
};
}
