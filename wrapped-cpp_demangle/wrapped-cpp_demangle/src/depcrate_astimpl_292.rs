// Generated macro for impl_292 (impl)
macro_rules! Depcrate_astimpl_292 {
() => {
// Module: crate::ast
// Provides: {"impl_292"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for LocalName { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { LocalName :: Relative (_ , None , _) => None , LocalName :: Relative (_ , Some (ref name) , _) | LocalName :: Default (_ , _ , ref name) => { name . get_leaf_name (subs) } } } }
};
}
