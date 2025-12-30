// Generated macro for impl_75 (impl)
macro_rules! Depcrate_chalk_extimpl_75 {
() => {
// Module: crate::chalk_ext
// Provides: {"impl_75"}
// Dependencies: {}
impl ProjectionTyExt for ProjectionTy { fn trait_ref (& self , db : & dyn HirDatabase) -> TraitRef { let generics = generics (db , from_assoc_type_id (self . associated_ty_id) . into ()) ; let parent_len = generics . parent_generics () . map_or (0 , | g | g . len_self ()) ; let substitution = Substitution :: from_iter (Interner , self . substitution . iter (Interner) . take (parent_len)) ; TraitRef { trait_id : to_chalk_trait_id (self . trait_ (db)) , substitution } } fn trait_ (& self , db : & dyn HirDatabase) -> TraitId { match from_assoc_type_id (self . associated_ty_id) . lookup (db) . container { ItemContainerId :: TraitId (it) => it , _ => panic ! ("projection ty without parent trait") , } } fn self_type_parameter (& self , db : & dyn HirDatabase) -> Ty { self . trait_ref (db) . self_type_parameter (Interner) } }
};
}
