// Generated macro for impl_104 (impl)
macro_rules! Depcrate_astimpl_104 {
() => {
// Module: crate::ast
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for UnscopedName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { UnscopedName :: Unqualified (ref name) | UnscopedName :: Std (ref name) => { name . get_leaf_name (subs) } } } }
};
}
