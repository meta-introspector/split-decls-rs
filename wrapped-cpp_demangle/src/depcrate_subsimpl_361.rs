// Generated macro for impl_361 (impl)
macro_rules! Depcrate_subsimpl_361 {
() => {
// Module: crate::subs
// Provides: {"impl_361"}
// Dependencies: {}
impl < 'a > ast :: GetLeafName < 'a > for Substitutable { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < ast :: LeafName < 'a > > { match * self { Substitutable :: UnscopedTemplateName (ref name) => name . get_leaf_name (subs) , Substitutable :: Prefix (ref prefix) => prefix . get_leaf_name (subs) , Substitutable :: Type (ref ty) => ty . get_leaf_name (subs) , _ => None , } } }
};
}
