// Generated macro for expand_function_arguments (function)
macro_rules! Depcrateexpand_function_arguments {
() => {
// Module: crate
// Provides: {"expand_function_arguments"}
// Dependencies: {}
fn expand_function_arguments (dest : & mut ItemFn , source : & ItemFn) { let to_merge_args = collect_template_args (source) ; for arg in dest . sig . inputs . iter_mut () { if let syn :: FnArg :: Typed (a) = arg { if let syn :: Pat :: Ident (ref id) = * a . pat { if let Some (source_arg) = resolve_template_arg (& to_merge_args , & id . ident) { merge_arg_attributes (& mut a . attrs , & source_arg . attrs) ; } } } } }
};
}
