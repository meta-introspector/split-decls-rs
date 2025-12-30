// Generated macro for impl_1012 (impl)
macro_rules! Depcrate_stream_stream_scanimpl_1012 {
() => {
// Module: crate::stream::stream::scan
// Provides: {"impl_1012"}
// Dependencies: {}
impl < St : Stream , S , Fut , F > Scan < St , S , Fut , F > { # [doc = " Checks if internal state is `None`."] fn is_done_taking (& self) -> bool { self . state . is_empty () } }
};
}
