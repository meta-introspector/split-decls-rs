// Generated macro for impl_215 (impl)
macro_rules! Depcrate_impls_primitivesimpl_215 {
() => {
// Module: crate::impls::primitives
// Provides: {"impl_215"}
// Dependencies: {}
impl < T > Format for [T] where T : Format , { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ("{=[?]}") } # [inline] fn _format_data (& self) { export :: fmt_slice (self) ; } }
};
}
