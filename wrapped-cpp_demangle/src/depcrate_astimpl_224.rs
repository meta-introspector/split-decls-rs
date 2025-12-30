// Generated macro for impl_224 (impl)
macro_rules! Depcrate_astimpl_224 {
() => {
// Module: crate::ast
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'subs > ArgScope < 'subs , 'subs > for UnnamedTypeName { fn leaf_name (& 'subs self) -> Result < LeafName < 'subs > > { Ok (LeafName :: UnnamedType (self)) } fn get_template_arg (& 'subs self , _ : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'subs self , _ : usize) -> Result < & 'subs Type > { Err (error :: Error :: BadFunctionArgReference) } }
};
}
