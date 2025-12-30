// Generated macro for impl_291 (impl)
macro_rules! Depcrate_astimpl_291 {
() => {
// Module: crate::ast
// Provides: {"impl_291"}
// Dependencies: {}
impl GetTemplateArgs for LocalName { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { LocalName :: Relative (_ , None , _) => None , LocalName :: Relative (_ , Some (ref name) , _) | LocalName :: Default (_ , _ , ref name) => { name . get_template_args (subs) } } } }
};
}
