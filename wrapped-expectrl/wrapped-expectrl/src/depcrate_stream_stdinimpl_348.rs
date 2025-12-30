// Generated macro for impl_348 (impl)
macro_rules! Depcrate_stream_stdinimpl_348 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_348"}
// Dependencies: {}
# [cfg (all (windows , feature = "polling" , not (feature = "async")))] impl Clone for Stdin { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } }
};
}
