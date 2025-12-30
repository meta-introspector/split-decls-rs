// Generated macro for impl_268 (impl)
macro_rules! Depcrate_deimpl_268 {
() => {
// Module: crate::de
// Provides: {"impl_268"}
// Dependencies: {}
# [doc (hidden)] impl de :: Error for Error { fn custom < T : Display > (msg : T) -> Self { ErrorKind :: Serde (msg . to_string ()) . without_position () } }
};
}
