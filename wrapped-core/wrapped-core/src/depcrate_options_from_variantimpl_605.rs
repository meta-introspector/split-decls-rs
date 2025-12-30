// Generated macro for impl_605 (impl)
macro_rules! Depcrate_options_from_variantimpl_605 {
() => {
// Module: crate::options::from_variant
// Provides: {"impl_605"}
// Dependencies: {}
impl FromVariantOptions { pub fn new (di : & DeriveInput) -> Result < Self > { (FromVariantOptions { base : OuterFrom :: start (di) ? , discriminant : Default :: default () , fields : Default :: default () , supports : Default :: default () , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) } }
};
}
