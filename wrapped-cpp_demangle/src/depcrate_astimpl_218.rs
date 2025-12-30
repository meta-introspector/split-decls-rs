// Generated macro for impl_218 (impl)
macro_rules! Depcrate_astimpl_218 {
() => {
// Module: crate::ast
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for ClassEnumType { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { ClassEnumType :: Named (ref name) | ClassEnumType :: ElaboratedStruct (ref name) | ClassEnumType :: ElaboratedUnion (ref name) | ClassEnumType :: ElaboratedEnum (ref name) => name . get_leaf_name (subs) , } } }
};
}
