// Generated macro for impl_101 (impl)
macro_rules! Depcrate_errorimpl_101 {
() => {
// Module: crate::error
// Provides: {"impl_101"}
// Dependencies: {}
impl de :: Error for Error { # [cold] fn custom < T : Display > (msg : T) -> Error { make_error (msg . to_string ()) } # [cold] fn invalid_type (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { Error :: custom (format_args ! ("invalid type: {}, expected {}" , JsonUnexpected (unexp) , exp ,)) } # [cold] fn invalid_value (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { Error :: custom (format_args ! ("invalid value: {}, expected {}" , JsonUnexpected (unexp) , exp ,)) } }
};
}
