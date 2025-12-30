// Generated macro for function_return_attr (function)
macro_rules! Depcrate_attributesfunction_return_attr {
() => {
// Module: crate::attributes
// Provides: {"function_return_attr"}
// Dependencies: {}
fn function_return_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let function_return_attr = match cx . sess () . opts . unstable_opts . function_return { FunctionReturn :: Keep => return None , FunctionReturn :: ThunkExtern => AttributeKind :: FnRetThunkExtern , } ; Some (function_return_attr . create_attr (cx . llcx)) }
};
}
