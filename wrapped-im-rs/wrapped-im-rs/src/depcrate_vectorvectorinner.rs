// Generated macro for VectorInner (enum)
macro_rules! Depcrate_vectorVectorInner {
() => {
// Module: crate::vector
// Provides: {"VectorInner"}
// Dependencies: {}
enum VectorInner < A > { Inline (RRBPool < A > , InlineArray < A , Rrb < A > >) , Single (RRBPool < A > , PoolRef < Chunk < A > >) , Full (RRBPool < A > , Rrb < A >) , }
};
}
