// Generated macro for impl_537 (impl)
macro_rules! Depcrate_options_from_attributesimpl_537 {
() => {
// Module: crate::options::from_attributes
// Provides: {"impl_537"}
// Dependencies: {}
impl ParseData for FromAttributesOptions { fn parse_variant (& mut self , variant : & syn :: Variant) -> Result < () > { self . base . parse_variant (variant) } fn parse_field (& mut self , field : & syn :: Field) -> Result < () > { self . base . parse_field (field) } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . base . validate_body (errors) ; } }
};
}
