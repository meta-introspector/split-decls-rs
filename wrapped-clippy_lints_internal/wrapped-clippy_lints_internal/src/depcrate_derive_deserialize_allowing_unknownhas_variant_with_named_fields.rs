// Generated macro for has_variant_with_named_fields (function)
macro_rules! Depcrate_derive_deserialize_allowing_unknownhas_variant_with_named_fields {
() => {
// Module: crate::derive_deserialize_allowing_unknown
// Provides: {"has_variant_with_named_fields"}
// Dependencies: {}
fn has_variant_with_named_fields (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let ty = tcx . type_of (def_id) . skip_binder () ; let rustc_middle :: ty :: Adt (adt_def , _) = ty . kind () else { return false ; } ; adt_def . variants () . iter () . any (| variant_def | variant_def . ctor . is_none ()) }
};
}
