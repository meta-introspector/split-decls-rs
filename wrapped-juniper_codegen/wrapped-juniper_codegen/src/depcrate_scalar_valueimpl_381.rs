// Generated macro for impl_381 (impl)
macro_rules! Depcrate_scalar_valueimpl_381 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_381"}
// Dependencies: {}
impl Parse for VariantAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < VariantAttr > { let mut out = Vec :: new () ; while ! input . is_empty () { let ident = input . parse :: < syn :: Ident > () ? ; let method = match ident . to_string () . as_str () { "to_int" => Method :: ToInt , "to_float" => Method :: ToFloat , "as_str" => Method :: AsStr , "to_string" => Method :: ToString , "to_bool" => Method :: ToBool , name => { return Err (err :: unknown_arg (& ident , name)) ; } } ; let expr = input . parse :: < token :: Eq > () . ok () . map (| _ | input . parse :: < syn :: ExprPath > ()) . transpose () ? ; out . push (SpanContainer :: new (ident . span () , expr . as_ref () . map (| e | e . span ()) , (method , expr) ,)) ; input . try_parse :: < token :: Comma > () ? ; } Ok (VariantAttr (out)) } }
};
}
