// Generated macro for contains_illegal_impl_trait_in_trait (function)
macro_rules! Depcrate_dyn_compatibilitycontains_illegal_impl_trait_in_trait {
() => {
// Module: crate::dyn_compatibility
// Provides: {"contains_illegal_impl_trait_in_trait"}
// Dependencies: {}
fn contains_illegal_impl_trait_in_trait < 'db > (db : & 'db dyn HirDatabase , sig : & EarlyBinder < 'db , Binder < 'db , rustc_type_ir :: FnSig < DbInterner < 'db > > > > ,) -> Option < MethodViolationCode > { struct OpaqueTypeCollector (FxHashSet < InternedOpaqueTyId >) ; impl < 'db > rustc_type_ir :: TypeVisitor < DbInterner < 'db > > for OpaqueTypeCollector { type Result = ControlFlow < () > ; fn visit_ty (& mut self , ty : < DbInterner < 'db > as rustc_type_ir :: Interner > :: Ty ,) -> Self :: Result { if let rustc_type_ir :: TyKind :: Alias (AliasTyKind :: Opaque , op) = ty . kind () { let id = match op . def_id { SolverDefId :: InternedOpaqueTyId (id) => id , _ => unreachable ! () , } ; self . 0 . insert (id) ; } ty . super_visit_with (self) } } let ret = sig . skip_binder () . output () ; let mut visitor = OpaqueTypeCollector (FxHashSet :: default ()) ; _ = ret . visit_with (& mut visitor) ; for opaque_ty in visitor . 0 { let impl_trait_id = db . lookup_intern_impl_trait_id (opaque_ty) ; if matches ! (impl_trait_id , ImplTraitId :: ReturnTypeImplTrait (..)) { return Some (MethodViolationCode :: ReferencesImplTraitInTrait) ; } } None }
};
}
