// Generated macro for has_supertrait (function)
macro_rules! Depcrate_tagged_traithas_supertrait {
() => {
// Module: crate::tagged_trait
// Provides: {"has_supertrait"}
// Dependencies: {}
fn has_supertrait (input : & ItemTrait , find : & str) -> bool { for supertrait in & input . supertraits { if let TypeParamBound :: Trait (trait_bound) = supertrait { if let TraitBoundModifier :: None = trait_bound . modifier { if trait_bound . path . is_ident (find) { return true ; } } } } false }
};
}
