// Generated macro for VClock (struct)
macro_rules! Depcrate_concurrency_vector_clockVClock {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"VClock"}
// Dependencies: {}
# [doc = " A vector clock for detecting data-races, this is conceptually"] # [doc = " a map from a vector index (and thus a thread id) to a timestamp."] # [doc = " The compare operations require that the invariant that the last"] # [doc = " element in the internal timestamp slice must not be a 0, hence"] # [doc = " all zero vector clocks are always represented by the empty slice;"] # [doc = " and allows for the implementation of compare operations to short"] # [doc = " circuit the calculation and return the correct result faster,"] # [doc = " also this means that there is only one unique valid length"] # [doc = " for each set of vector clock values and hence the PartialEq"] # [doc = " and Eq derivations are correct."] # [doc = ""] # [doc = " This means we cannot represent a clock where the last entry is a timestamp-0 read that occurs"] # [doc = " because of a retag. That's fine, all it does is risk wrong diagnostics in a extreme corner case."] # [derive (PartialEq , Eq , Default , Debug)] pub struct VClock (SmallVec < [VTimestamp ; SMALL_VECTOR] >) ;
};
}
