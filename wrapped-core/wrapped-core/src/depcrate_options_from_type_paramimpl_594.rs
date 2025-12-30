// Generated macro for impl_594 (impl)
macro_rules! Depcrate_options_from_type_paramimpl_594 {
() => {
// Module: crate::options::from_type_param
// Provides: {"impl_594"}
// Dependencies: {}
impl ParseData for FromTypeParamOptions { fn parse_variant (& mut self , variant : & syn :: Variant) -> Result < () > { self . base . parse_variant (variant) } fn parse_field (& mut self , field : & syn :: Field) -> Result < () > { match field . ident . as_ref () . map (| v | v . to_string ()) . as_deref () { Some ("bounds") => { self . bounds . clone_from (& field . ident) ; Ok (()) } Some ("default") => { self . default . clone_from (& field . ident) ; Ok (()) } _ => self . base . parse_field (field) , } } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . base . validate_body (errors) ; } }
};
}
