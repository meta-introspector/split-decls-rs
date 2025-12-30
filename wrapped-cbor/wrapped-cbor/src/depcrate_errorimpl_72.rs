// Generated macro for impl_72 (impl)
macro_rules! Depcrate_errorimpl_72 {
() => {
// Module: crate::error
// Provides: {"impl_72"}
// Dependencies: {}
impl de :: Error for Error { fn custom < T : fmt :: Display > (msg : T) -> Error { Error :: message (msg) } fn invalid_type (unexp : de :: Unexpected < '_ > , exp : & dyn de :: Expected) -> Error { if let de :: Unexpected :: Unit = unexp { Error :: custom (format_args ! ("invalid type: null, expected {}" , exp)) } else { Error :: custom (format_args ! ("invalid type: {}, expected {}" , unexp , exp)) } } }
};
}
