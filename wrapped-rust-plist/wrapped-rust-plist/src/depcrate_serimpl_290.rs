// Generated macro for impl_290 (impl)
macro_rules! Depcrate_serimpl_290 {
() => {
// Module: crate::ser
// Provides: {"impl_290"}
// Dependencies: {}
# [doc (hidden)] impl ser :: Error for Error { fn custom < T : Display > (msg : T) -> Self { ErrorKind :: Serde (msg . to_string ()) . without_position () } }
};
}
