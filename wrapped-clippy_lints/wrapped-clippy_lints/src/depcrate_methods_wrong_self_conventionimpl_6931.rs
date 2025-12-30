// Generated macro for impl_6931 (impl)
macro_rules! Depcrate_methods_wrong_self_conventionimpl_6931 {
() => {
// Module: crate::methods::wrong_self_convention
// Provides: {"impl_6931"}
// Dependencies: {}
impl Convention { # [must_use] fn check < 'tcx > (& self , cx : & LateContext < 'tcx > , self_ty : Ty < 'tcx > , other : & str , implements_trait : bool , is_trait_item : bool ,) -> bool { match * self { Self :: Eq (this) => this == other , Self :: StartsWith (this) => other . starts_with (this) && this != other , Self :: EndsWith (this) => other . ends_with (this) && this != other , Self :: NotEndsWith (this) => ! Self :: EndsWith (this) . check (cx , self_ty , other , implements_trait , is_trait_item) , Self :: IsSelfTypeCopy (is_true) => is_true == is_copy (cx , self_ty) , Self :: ImplementsTrait (is_true) => is_true == implements_trait , Self :: IsTraitItem (is_true) => is_true == is_trait_item , } } }
};
}
