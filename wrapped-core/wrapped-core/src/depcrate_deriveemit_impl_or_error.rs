// Generated macro for emit_impl_or_error (macro)
macro_rules! Depcrate_deriveemit_impl_or_error {
() => {
// Module: crate::derive
// Provides: {"emit_impl_or_error"}
// Dependencies: {}
# [doc = " Run an expression which returns a `darling::Result`, then either return the tokenized"] # [doc = " representation of the `Ok` value, or the tokens of the compiler errors in the `Err` case."] macro_rules ! emit_impl_or_error { ($ e : expr) => { match $ e { Ok (val) => val . into_token_stream () , Err (err) => err . write_errors () , } } ; }
};
}
