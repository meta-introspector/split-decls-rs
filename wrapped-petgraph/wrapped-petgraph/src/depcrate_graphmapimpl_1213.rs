// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_graphmapimpl_1213 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1213"}
// Dependencies: {}
# [doc = " Create a new empty `GraphMap`."] impl < N , E , Ty , S > Default for GraphMap < N , E , Ty , S > where S : BuildHasher + Default , { fn default () -> Self { GraphMap :: with_capacity (0 , 0) } }
};
}
