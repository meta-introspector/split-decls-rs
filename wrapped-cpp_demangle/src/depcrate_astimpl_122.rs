// Generated macro for impl_122 (impl)
macro_rules! Depcrate_astimpl_122 {
() => {
// Module: crate::ast
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for Prefix { fn get_leaf_name (& 'a self , subs : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { Prefix :: Nested (ref prefix , ref name) => name . get_leaf_name (subs) . or_else (| | prefix . get_leaf_name (subs)) , Prefix :: Unqualified (ref name) => name . get_leaf_name (subs) , Prefix :: Template (ref prefix , _) => prefix . get_leaf_name (subs) , Prefix :: DataMember (_ , ref name) => name . get_leaf_name (subs) , Prefix :: TemplateParam (_) | Prefix :: Decltype (_) => None , } } }
};
}
