// Generated macro for impl_176 (impl)
macro_rules! Depcrate_astimpl_176 {
() => {
// Module: crate::ast
// Provides: {"impl_176"}
// Dependencies: {}
impl GetTemplateArgs for TypeHandle { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { subs . get_type (self) . and_then (| ty | ty . get_template_args (subs)) } }
};
}
