// Generated macro for impl_606 (impl)
macro_rules! Depcrate_options_from_variantimpl_606 {
() => {
// Module: crate::options::from_variant
// Provides: {"impl_606"}
// Dependencies: {}
impl < 'a > From < & 'a FromVariantOptions > for FromVariantImpl < 'a > { fn from (v : & 'a FromVariantOptions) -> Self { FromVariantImpl { base : (& v . base . container) . into () , ident : v . base . ident . as_ref () , discriminant : v . discriminant . as_ref () , fields : v . fields . as_ref () , attr_names : & v . base . attr_names , forward_attrs : v . base . as_forward_attrs () , from_ident : v . base . from_ident , supports : v . supports . as_ref () , } } }
};
}
