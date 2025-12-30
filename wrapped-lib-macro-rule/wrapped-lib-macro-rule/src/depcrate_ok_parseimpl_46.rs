// Generated macro for impl_46 (impl)
macro_rules! Depcrate_ok_parseimpl_46 {
() => {
// Module: crate::ok_parse
// Provides: {"impl_46"}
// Dependencies: {}
impl < T > ParseResultBase < T > for OkParse < T > { fn handle_failure (& mut self , _tracker : & mut DynMTrackerTrait ! ()) { } fn is_ok (& self) -> bool { true } fn unwrap (self) -> Option < T > { Some (self . 0) } }
};
}
