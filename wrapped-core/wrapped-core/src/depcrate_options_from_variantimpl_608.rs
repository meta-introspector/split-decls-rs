// Generated macro for impl_608 (impl)
macro_rules! Depcrate_options_from_variantimpl_608 {
() => {
// Module: crate::options::from_variant
// Provides: {"impl_608"}
// Dependencies: {}
impl ParseData for FromVariantOptions { fn parse_field (& mut self , field : & Field) -> Result < () > { match field . ident . as_ref () . map (| v | v . to_string ()) . as_deref () { Some ("discriminant") => { self . discriminant . clone_from (& field . ident) ; Ok (()) } Some ("fields") => { self . fields . clone_from (& field . ident) ; Ok (()) } _ => self . base . parse_field (field) , } } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . base . validate_body (errors) ; } }
};
}
