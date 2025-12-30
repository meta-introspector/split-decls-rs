// Generated macro for impl_123 (impl)
macro_rules! Depcrate_astimpl_123 {
() => {
// Module: crate::ast
// Provides: {"impl_123"}
// Dependencies: {}
impl GetTemplateArgs for PrefixHandle { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { PrefixHandle :: BackReference (idx) => { if let Some (& Substitutable :: Prefix (ref p)) = subs . get (idx) { p . get_template_args (subs) } else { None } } PrefixHandle :: NonSubstitution (NonSubstitution (idx)) => { if let Some (& Substitutable :: Prefix (ref p)) = subs . get_non_substitution (idx) { p . get_template_args (subs) } else { None } } _ => None , } } }
};
}
