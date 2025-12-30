// Generated macro for RayonJoin (enum)
macro_rules! Depcrate_joinRayonJoin {
() => {
// Module: crate::join
// Provides: {"RayonJoin"}
// Dependencies: {}
# [doc = " The Rayon-based implementation of `Join`. The left and right sides are"] # [doc = " executed on the Rayon thread pool, potentially in parallel. This"] # [doc = " implementation is gated by the `rayon` feature, which is off by default."] # [doc = ""] # [doc = " See the [`join` module docs](index.html) for more details."] # [cfg (feature = "rayon")] pub enum RayonJoin { }
};
}
