// Generated macro for tait_defining_bodies (function)
macro_rules! Depcrate_infer_opaquestait_defining_bodies {
() => {
// Module: crate::infer::opaques
// Provides: {"tait_defining_bodies"}
// Dependencies: {}
fn tait_defining_bodies (db : & dyn HirDatabase , loc : & AssocItemLoc < ast :: TypeAlias > ,) -> Vec < DefWithBodyId > { let from_assoc_items = | assoc_items : & [(Name , AssocItemId)] | { assoc_items . iter () . filter_map (| & (_ , assoc_id) | match assoc_id { AssocItemId :: FunctionId (it) => Some (it . into ()) , AssocItemId :: ConstId (it) => Some (it . into ()) , AssocItemId :: TypeAliasId (_) => None , }) . collect () } ; match loc . container { ItemContainerId :: ImplId (impl_id) => { if db . impl_signature (impl_id) . target_trait . is_some () { return from_assoc_items (& impl_id . impl_items (db) . items) ; } } ItemContainerId :: TraitId (trait_id) => { return from_assoc_items (& trait_id . trait_items (db) . items) ; } _ => { } } Vec :: new () }
};
}
