// Generated macro for allowed_lts_from (function)
macro_rules! Depcrate_lifetimesallowed_lts_from {
() => {
// Module: crate::lifetimes
// Provides: {"allowed_lts_from"}
// Dependencies: {}
fn allowed_lts_from (named_generics : & [GenericParam < '_ >]) -> FxIndexSet < LocalDefId > { named_generics . iter () . filter_map (| par | { if let GenericParamKind :: Lifetime { .. } = par . kind { Some (par . def_id) } else { None } }) . collect () }
};
}
