// Generated macro for bounds_reference_self (function)
macro_rules! Depcrate_dyn_compatibilitybounds_reference_self {
() => {
// Module: crate::dyn_compatibility
// Provides: {"bounds_reference_self"}
// Dependencies: {}
fn bounds_reference_self (db : & dyn HirDatabase , trait_ : TraitId) -> bool { let trait_data = trait_ . trait_items (db) ; trait_data . items . iter () . filter_map (| (_ , it) | match * it { AssocItemId :: TypeAliasId (id) => Some (associated_ty_item_bounds (db , id)) , _ => None , }) . any (| bounds | { bounds . skip_binder () . iter () . any (| pred | match pred . skip_binder () { rustc_type_ir :: ExistentialPredicate :: Trait (it) => it . args . iter () . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , AllowSelfProjection :: Yes) }) , rustc_type_ir :: ExistentialPredicate :: Projection (it) => it . args . iter () . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , AllowSelfProjection :: Yes) }) , rustc_type_ir :: ExistentialPredicate :: AutoTrait (_) => false , }) }) }
};
}
