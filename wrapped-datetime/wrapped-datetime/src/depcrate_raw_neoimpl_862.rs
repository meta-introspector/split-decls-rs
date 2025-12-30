// Generated macro for impl_862 (impl)
macro_rules! Depcrate_raw_neoimpl_862 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_862"}
// Dependencies: {}
impl RawOptions { # [inline] pub (crate) fn length (self) -> Length { self . length . unwrap_or_else (| | { debug_assert ! (false , "length not set in a formatter that needs it") ; Default :: default () }) } }
};
}
