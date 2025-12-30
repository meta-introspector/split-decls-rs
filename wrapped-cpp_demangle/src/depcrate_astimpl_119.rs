// Generated macro for impl_119 (impl)
macro_rules! Depcrate_astimpl_119 {
() => {
// Module: crate::ast
// Provides: {"impl_119"}
// Dependencies: {}
impl GetTemplateArgs for Prefix { fn get_template_args < 'a > (& 'a self , _ : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { Prefix :: Template (_ , ref args) => Some (args) , Prefix :: Unqualified (_) | Prefix :: Nested (_ , _) | Prefix :: TemplateParam (_) | Prefix :: Decltype (_) | Prefix :: DataMember (_ , _) => None , } } }
};
}
