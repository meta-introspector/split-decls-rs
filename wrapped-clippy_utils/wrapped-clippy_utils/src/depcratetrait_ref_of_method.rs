// Generated macro for trait_ref_of_method (function)
macro_rules! Depcratetrait_ref_of_method {
() => {
// Module: crate
// Provides: {"trait_ref_of_method"}
// Dependencies: {}
# [doc = " Gets the `hir::TraitRef` of the trait the given method is implemented for."] # [doc = ""] # [doc = " Use this if you want to find the `TraitRef` of the `Add` trait in this example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " struct Point(isize, isize);"] # [doc = ""] # [doc = " impl std::ops::Add for Point {"] # [doc = "     type Output = Self;"] # [doc = ""] # [doc = "     fn add(self, other: Self) -> Self {"] # [doc = "         Point(0, 0)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn trait_ref_of_method < 'tcx > (cx : & LateContext < 'tcx > , owner : OwnerId) -> Option < & 'tcx TraitRef < 'tcx > > { if let Node :: Item (item) = cx . tcx . hir_node (cx . tcx . hir_owner_parent (owner)) && let ItemKind :: Impl (impl_) = & item . kind && let Some (of_trait) = impl_ . of_trait { return Some (& of_trait . trait_ref) ; } None }
};
}
