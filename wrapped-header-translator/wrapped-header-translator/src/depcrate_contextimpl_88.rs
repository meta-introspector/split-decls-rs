// Generated macro for impl_88 (impl)
macro_rules! Depcrate_contextimpl_88 {
() => {
// Module: crate::context
// Provides: {"impl_88"}
// Dependencies: {}
impl MacroEntity { pub fn from_entity (entity : & Entity < '_ > , context : & Context < '_ > , is_definition : bool) -> Self { let definition = entity . get_definition () ; let macro_arguments = if ! is_definition { if entity . get_name () . unwrap () . contains ("BRIDGED") { parse_macro_arguments (& get_argument_tokens (entity)) } else { Default :: default () } } else { Default :: default () } ; Self { id : ItemIdentifier :: new (definition . as_ref () . unwrap_or (entity) , context) , is_function_like : entity . is_function_like_macro () , macro_arguments , value : None , } } }
};
}
