// Generated macro for impl_252 (impl)
macro_rules! Depcrate_astimpl_252 {
() => {
// Module: crate::ast
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'subs > ArgScope < 'subs , 'subs > for TemplateArgs { fn leaf_name (& 'subs self) -> Result < LeafName < 'subs > > { Err (error :: Error :: BadLeafNameReference) } fn get_template_arg (& 'subs self , idx : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { self . 0 . get (idx) . ok_or (error :: Error :: BadTemplateArgReference) . map (| v | (v , self)) } fn get_function_arg (& 'subs self , _ : usize) -> Result < & 'subs Type > { Err (error :: Error :: BadFunctionArgReference) } }
};
}
