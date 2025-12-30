// Generated macro for impl_682 (impl)
macro_rules! Depcrate_usage_generics_extimpl_682 {
() => {
// Module: crate::usage::generics_ext
// Provides: {"impl_682"}
// Dependencies: {}
impl GenericsExt for Generics { fn declared_lifetimes (& self) -> LifetimeSet { self . lifetimes () . map (| lt | lt . lifetime . clone ()) . collect () } fn declared_type_params (& self) -> IdentSet { self . type_params () . map (| tp | tp . ident . clone ()) . collect () } }
};
}
