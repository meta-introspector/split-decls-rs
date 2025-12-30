// Generated macro for resolve_container (function)
macro_rules! Depcrate_dynamic_resolveresolve_container {
() => {
// Module: crate::dynamic::resolve
// Provides: {"resolve_container"}
// Dependencies: {}
pub (crate) async fn resolve_container (schema : & Schema , object : & Object , ctx : & ContextSelectionSet < '_ > , parent_value : & FieldValue < '_ > , serial : bool ,) -> ServerResult < Option < Value > > { let mut fields = Vec :: new () ; collect_fields (& mut fields , schema , object , ctx , parent_value) ? ; let res = if ! serial { futures_util :: future :: try_join_all (fields) . await ? } else { let mut results = Vec :: with_capacity (fields . len ()) ; for field in fields { results . push (field . await ?) ; } results } ; Ok (Some (create_value_object (res))) }
};
}
