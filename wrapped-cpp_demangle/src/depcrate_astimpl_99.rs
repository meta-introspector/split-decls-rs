// Generated macro for impl_99 (impl)
macro_rules! Depcrate_astimpl_99 {
() => {
// Module: crate::ast
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for Name { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Name :: UnscopedTemplate (ref templ , _) => templ . get_leaf_name (subs) , Name :: Nested (ref nested) => nested . get_leaf_name (subs) , Name :: Unscoped (ref unscoped) => unscoped . get_leaf_name (subs) , Name :: Local (ref local) => local . get_leaf_name (subs) , } } }
};
}
