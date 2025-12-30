// Generated macro for impl_80 (impl)
macro_rules! Depcrate_astimpl_80 {
() => {
// Module: crate::ast
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for NonSubstitution { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { subs . get_non_substitution (self . 0) . and_then (| ns | ns . get_leaf_name (subs)) } }
};
}
