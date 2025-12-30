// Generated macro for impl_115 (impl)
macro_rules! Depcrate_astimpl_115 {
() => {
// Module: crate::ast
// Provides: {"impl_115"}
// Dependencies: {}
impl GetTemplateArgs for NestedName { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { NestedName :: Template (_ , _ , ref prefix) | NestedName :: TemplateExplicitObject (ref prefix , _) => prefix . get_template_args (subs) , _ => None , } } }
};
}
