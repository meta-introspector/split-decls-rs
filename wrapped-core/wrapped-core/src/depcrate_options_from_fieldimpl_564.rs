// Generated macro for impl_564 (impl)
macro_rules! Depcrate_options_from_fieldimpl_564 {
() => {
// Module: crate::options::from_field
// Provides: {"impl_564"}
// Dependencies: {}
impl ParseData for FromFieldOptions { fn parse_variant (& mut self , variant : & syn :: Variant) -> Result < () > { self . base . parse_variant (variant) } fn parse_field (& mut self , field : & syn :: Field) -> Result < () > { match field . ident . as_ref () . map (| v | v . to_string ()) . as_deref () { Some ("vis") => { self . vis . clone_from (& field . ident) ; Ok (()) } Some ("ty") => { self . ty . clone_from (& field . ident) ; Ok (()) } _ => self . base . parse_field (field) , } } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . base . validate_body (errors) ; } }
};
}
