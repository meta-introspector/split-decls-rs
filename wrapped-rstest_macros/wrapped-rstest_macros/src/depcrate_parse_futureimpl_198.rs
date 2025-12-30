// Generated macro for impl_198 (impl)
macro_rules! Depcrate_parse_futureimpl_198 {
() => {
// Module: crate::parse::future
// Provides: {"impl_198"}
// Dependencies: {}
impl Validator < FnArg > for FutureBuilder { fn validate (arg : & FnArg) -> syn :: Result < () > { arg . as_future_impl_type () . map (| _ | ()) . ok_or_else (| | { syn :: Error :: new_spanned (arg . maybe_type () . unwrap () . into_token_stream () , "This type cannot used to generate impl Future." . to_owned () ,) }) } }
};
}
