// Generated macro for impl_15 (impl)
macro_rules! Depcrate_helperimpl_15 {
() => {
// Module: crate::helper
// Provides: {"impl_15"}
// Dependencies: {}
impl From < Context > for NextAction { fn from (ctx : Context) -> Self { let mut buf = Vec :: < u8 > :: new () ; ctx . write_to (& mut buf) . expect ("cannot fail") ; NextAction { previous_output : buf . into () , } } }
};
}
