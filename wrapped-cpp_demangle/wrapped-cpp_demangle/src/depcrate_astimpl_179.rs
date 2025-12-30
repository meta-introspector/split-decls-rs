// Generated macro for impl_179 (impl)
macro_rules! Depcrate_astimpl_179 {
() => {
// Module: crate::ast
// Provides: {"impl_179"}
// Dependencies: {}
impl GetTemplateArgs for Type { fn get_template_args < 'a > (& 'a self , subs : & 'a SubstitutionTable) -> Option < & 'a TemplateArgs > { match * self { Type :: VendorExtension (_ , Some (ref args) , _) | Type :: TemplateTemplate (_ , ref args) => { Some (args) } Type :: PointerTo (ref ty) | Type :: LvalueRef (ref ty) | Type :: RvalueRef (ref ty) => { ty . get_template_args (subs) } _ => None , } } }
};
}
