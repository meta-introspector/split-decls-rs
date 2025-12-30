// Generated macro for impl_116 (impl)
macro_rules! Depcrate_resolver_inline_expressionimpl_116 {
() => {
// Module: crate::resolver::inline_expression
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'bundle > ResolveValue < 'bundle > for ast :: InlineExpression < & 'bundle str > { fn resolve < 'ast , 'args , 'errors , R , M > (& 'ast self , scope : & mut Scope < 'bundle , 'ast , 'args , 'errors , R , M > ,) -> FluentValue < 'bundle > where R : Borrow < FluentResource > , M : MemoizerKind , { match self { Self :: StringLiteral { value } => unescape_unicode_to_string (value) . into () , Self :: NumberLiteral { value } => FluentValue :: try_number (value) , Self :: VariableReference { id } => { if let Some (local_args) = & scope . local_args { if let Some (arg) = local_args . get (id . name) { return arg . clone () ; } } else if let Some (arg) = scope . args . and_then (| args | args . get (id . name)) { return arg . into_owned () ; } if scope . local_args . is_none () { scope . add_error (self . into ()) ; } FluentValue :: Error } Self :: FunctionReference { id , arguments } => { let (resolved_positional_args , resolved_named_args) = scope . get_arguments (Some (arguments)) ; let func = scope . bundle . get_entry_function (id . name) ; if let Some (func) = func { let result = func (resolved_positional_args . as_slice () , & resolved_named_args) ; result } else { FluentValue :: Error } } _ => { let mut result = String :: new () ; self . write (& mut result , scope) . expect ("Failed to write") ; result . into () } } } }
};
}
