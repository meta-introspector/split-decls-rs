// Generated macro for impl_179 (impl)
macro_rules! Depcrate_lowerimpl_179 {
() => {
// Module: crate::lower
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'db > ImplTraitId < 'db > { # [inline] pub fn predicates (self , db : & 'db dyn HirDatabase) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { let (impl_traits , idx) = match self { ImplTraitId :: ReturnTypeImplTrait (owner , idx) => { (ImplTraits :: return_type_impl_traits (db , owner) , idx) } ImplTraitId :: TypeAliasImplTrait (owner , idx) => { (ImplTraits :: type_alias_impl_traits (db , owner) , idx) } } ; impl_traits . as_deref () . expect ("owner should have opaque type") . as_ref () . map_bound (| it | & * it . impl_traits [idx] . predicates) } }
};
}
