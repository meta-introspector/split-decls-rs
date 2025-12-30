// Generated macro for maybe_small (module)
macro_rules! Depcratemaybe_small {
() => {
// Module: crate
// Provides: {"maybe_small"}
// Dependencies: {}
# [cfg (not (feature = "smallvec"))] mod maybe_small { pub type Vec < T > = alloc :: vec :: Vec < T > ; pub type IntoIter < T > = alloc :: vec :: IntoIter < T > ; }
};
}
