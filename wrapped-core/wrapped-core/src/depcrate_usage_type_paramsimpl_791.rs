// Generated macro for impl_791 (impl)
macro_rules! Depcrate_usage_type_paramsimpl_791 {
() => {
// Module: crate::usage::type_params
// Provides: {"impl_791"}
// Dependencies: {}
impl UsesTypeParams for syn :: Path { fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > { if self . segments . is_empty () { return Default :: default () ; } let ident_hits = if self . leading_colon . is_none () { self . segments [0] . ident . uses_type_params (options , type_set) } else { Default :: default () } ; self . segments . iter () . fold (ident_hits , | state , segment | { union_in_place (state , segment . arguments . uses_type_params (options , type_set)) }) } }
};
}
