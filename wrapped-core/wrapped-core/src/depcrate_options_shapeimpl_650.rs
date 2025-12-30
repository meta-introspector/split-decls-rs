// Generated macro for impl_650 (impl)
macro_rules! Depcrate_options_shapeimpl_650 {
() => {
// Module: crate::options::shape
// Provides: {"impl_650"}
// Dependencies: {}
impl DeriveInputShapeSet { fn validator_fn_ident (& self) -> syn :: Ident { syn :: Ident :: new ("__validate_body" , Span :: call_site ()) } pub fn validator_path (& self) -> syn :: Path { if self . any { parse_quote ! (:: darling :: export :: Ok) } else { self . validator_fn_ident () . into () } } }
};
}
