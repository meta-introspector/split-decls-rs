// Generated macro for impl_136 (impl)
macro_rules! Depcrate_astimpl_136 {
() => {
// Module: crate::ast
// Provides: {"impl_136"}
// Dependencies: {}
impl < 'subs > ArgScope < 'subs , 'subs > for SourceName { fn leaf_name (& 'subs self) -> Result < LeafName < 'subs > > { Ok (LeafName :: SourceName (self)) } fn get_template_arg (& 'subs self , _ : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'subs self , _ : usize) -> Result < & 'subs Type > { Err (error :: Error :: BadFunctionArgReference) } }
};
}
