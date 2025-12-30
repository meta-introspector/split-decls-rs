// Generated macro for impl_100 (impl)
macro_rules! Depcrate_common_parseimpl_100 {
() => {
// Module: crate::common::parse
// Provides: {"impl_100"}
// Dependencies: {}
impl VisitMut for ReplaceWithDefaults < '_ > { fn visit_generic_argument_mut (& mut self , arg : & mut syn :: GenericArgument) { match arg { syn :: GenericArgument :: Lifetime (lf) => { * lf = parse_quote ! { 'static } ; } syn :: GenericArgument :: Type (ty) => { let is_generic = self . 0 . params . iter () . filter_map (| par | match par { syn :: GenericParam :: Type (ty) => Some (& ty . ident) , _ => None , }) . any (| par | { let par = quote ! { # par } . to_string () ; let ty = quote ! { # ty } . to_string () ; par == ty }) ; if is_generic { * ty = parse_quote ! (:: juniper :: DefaultScalarValue) ; } } _ => { } } } }
};
}
