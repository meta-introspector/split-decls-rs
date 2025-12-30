// Generated macro for impl_642 (impl)
macro_rules! Depcrate_options_outer_fromimpl_642 {
() => {
// Module: crate::options::outer_from
// Provides: {"impl_642"}
// Dependencies: {}
impl ParseData for OuterFrom { fn parse_field (& mut self , field : & Field) -> Result < () > { match field . ident . as_ref () . map (| v | v . to_string ()) . as_deref () { Some ("ident") => { self . ident . clone_from (& field . ident) ; Ok (()) } Some ("attrs") => { self . attrs = ForwardedField :: from_field (field) . map (Some) ? ; Ok (()) } _ => self . container . parse_field (field) , } } fn validate_body (& self , errors : & mut crate :: error :: Accumulator) { self . container . validate_body (errors) ; if let Some (attrs) = & self . attrs { if self . forward_attrs . is_none () { let container_name = match & self . container . data { Data :: Enum (_) => "enum" , Data :: Struct (_) => "struct" , } ; errors . push (Error :: custom (format ! ("field will not be populated because `forward_attrs` is not set on the {}" , container_name)) . with_span (& attrs . ident) ,) ; } } if let Some (ForwardAttrsFilter :: Only (fwd)) = & self . forward_attrs { for path in fwd . intersection (& self . attr_names) { errors . push (Error :: custom (format ! ("attribute path `{}` will not be forwarded because it is also listed in `attributes`" , path . to_token_stream ())) . with_span (path) ,) ; } } ; } }
};
}
