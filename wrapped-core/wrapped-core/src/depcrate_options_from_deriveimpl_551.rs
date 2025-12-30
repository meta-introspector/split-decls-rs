// Generated macro for impl_551 (impl)
macro_rules! Depcrate_options_from_deriveimpl_551 {
() => {
// Module: crate::options::from_derive
// Provides: {"impl_551"}
// Dependencies: {}
impl ParseData for FdiOptions { fn parse_variant (& mut self , variant : & syn :: Variant) -> Result < () > { self . base . parse_variant (variant) } fn parse_field (& mut self , field : & syn :: Field) -> Result < () > { match field . ident . as_ref () . map (| v | v . to_string ()) . as_deref () { Some ("vis") => { self . vis . clone_from (& field . ident) ; Ok (()) } Some ("data") => { self . data = ForwardedField :: from_field (field) . map (Some) ? ; Ok (()) } Some ("generics") => { self . generics = ForwardedField :: from_field (field) . map (Some) ? ; Ok (()) } _ => self . base . parse_field (field) , } } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . base . validate_body (errors) ; } }
};
}
