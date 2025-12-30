// Generated macro for impl_313 (impl)
macro_rules! Depcrate_astimpl_313 {
() => {
// Module: crate::ast
// Provides: {"impl_313"}
// Dependencies: {}
impl < 'a > ArgScope < 'a , 'a > for WellKnownComponent { fn leaf_name (& 'a self) -> Result < LeafName < 'a > > { Ok (LeafName :: WellKnownComponent (self)) } fn get_template_arg (& 'a self , _ : usize) -> Result < (& 'a TemplateArg , & 'a TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'a self , _ : usize) -> Result < & 'a Type > { Err (error :: Error :: BadFunctionArgReference) } }
};
}
