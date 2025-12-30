// Generated macro for impl_22 (impl)
macro_rules! Depcrate_queriesimpl_22 {
() => {
// Module: crate::queries
// Provides: {"impl_22"}
// Dependencies: {}
impl ToTokens for InputSetter { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & mut self . signature . clone () ; let ty = & self . return_type ; let fn_ident = & sig . ident ; let create_data_ident = & self . create_data_ident ; let setter_ident = format_ident ! ("set_{}" , fn_ident) ; sig . ident = setter_ident . clone () ; let value_argument : PatType = parse_quote ! (__value : # ty) ; sig . inputs . push (FnArg :: Typed (value_argument . clone ())) ; let mut_receiver : Receiver = parse_quote ! (& mut self) ; if let Some (og) = sig . inputs . first_mut () { * og = FnArg :: Receiver (mut_receiver) } sig . output = ReturnType :: Default ; let value = & value_argument . pat ; let method = quote ! { # sig { use salsa :: Setter ; let data = # create_data_ident (self) ; data .# setter_ident (self) . to (Some (# value)) ; } } ; method . to_tokens (tokens) ; } }
};
}
