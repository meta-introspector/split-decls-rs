// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl VisitMut for RemoveAttrsFromTraitMethods { fn visit_item_trait_mut (& mut self , i : & mut syn :: ItemTrait) { for item in & mut i . items { if let TraitItem :: Fn (trait_item_fn) = item { trait_item_fn . attrs = vec ! [] ; } } } }
};
}
