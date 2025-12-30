// Generated macro for impl_180 (impl)
macro_rules! Depcrate_astimpl_180 {
() => {
// Module: crate::ast
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for Type { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Type :: ClassEnum (ref cls_enum_ty) => cls_enum_ty . get_leaf_name (subs) , _ => None , } } }
};
}
