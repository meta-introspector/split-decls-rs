// Generated macro for impl_48 (impl)
macro_rules! Depcrate_internals_attributes_fieldimpl_48 {
() => {
// Module: crate::internals::attributes::field
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "schema")] impl Attributes { fn check_schema (& self , attr : & Attribute) -> Result < () , syn :: Error > { if let Some (ref schema) = self . schema { if self . skip && schema . params . is_some () { return Err (syn :: Error :: new_spanned (attr , format ! ("`{}` cannot be used at the same time as `{}({})`" , SKIP . 0 , SCHEMA . 0 , PARAMS . 1) ,)) ; } if self . skip && schema . with_funcs . is_some () { return Err (syn :: Error :: new_spanned (attr , format ! ("`{}` cannot be used at the same time as `{}({})`" , SKIP . 0 , SCHEMA . 0 , WITH_FUNCS . 1) ,)) ; } } Ok (()) } pub (crate) fn needs_schema_params_derive (& self) -> bool { if let Some (ref schema) = self . schema { if schema . params . is_some () { return false ; } } true } pub (crate) fn schema_declaration (& self) -> Option < syn :: ExprPath > { self . schema . as_ref () . and_then (| schema | { schema . with_funcs . as_ref () . and_then (| with_funcs | with_funcs . declaration . clone ()) }) } pub (crate) fn schema_definitions (& self) -> Option < syn :: ExprPath > { self . schema . as_ref () . and_then (| schema | { schema . with_funcs . as_ref () . and_then (| with_funcs | with_funcs . definitions . clone ()) }) } }
};
}
