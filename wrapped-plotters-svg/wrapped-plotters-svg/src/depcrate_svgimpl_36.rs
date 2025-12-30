// Generated macro for impl_36 (impl)
macro_rules! Depcrate_svgimpl_36 {
() => {
// Module: crate::svg
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > AttrWriter < 'a , Value > { fn write_value (self , value : impl FormatEscaped) { self . buf . push_str ("=\"") ; FormatEscaped :: format_escaped (self . buf , value) ; self . buf . push ('"') ; } }
};
}
