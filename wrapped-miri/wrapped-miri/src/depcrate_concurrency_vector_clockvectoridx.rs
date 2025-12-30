// Generated macro for VectorIdx (struct)
macro_rules! Depcrate_concurrency_vector_clockVectorIdx {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"VectorIdx"}
// Dependencies: {}
# [doc = " A vector clock index, this is associated with a thread id"] # [doc = " but in some cases one vector index may be shared with"] # [doc = " multiple thread ids if it's safe to do so."] # [derive (Clone , Copy , Debug , PartialOrd , Ord , PartialEq , Eq , Hash)] pub (super) struct VectorIdx (u32) ;
};
}
