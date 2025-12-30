// Generated macro for impl_854 (impl)
macro_rules! Depcrate_rc_test_objectimpl_854 {
() => {
// Module: crate::rc::test_object
// Provides: {"impl_854"}
// Dependencies: {}
impl RcTestObject { # [doc (hidden)] # [allow (dead_code)] pub (crate) fn new () -> Retained < Self > { unsafe { Retained :: from_raw (msg_send ! [Self :: class () , new]) } . unwrap () } }
};
}
