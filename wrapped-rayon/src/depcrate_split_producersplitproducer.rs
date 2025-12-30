// Generated macro for SplitProducer (struct)
macro_rules! Depcrate_split_producerSplitProducer {
() => {
// Module: crate::split_producer
// Provides: {"SplitProducer"}
// Dependencies: {}
# [doc = " Common producer for splitting on a predicate."] pub (super) struct SplitProducer < 'p , P , V , const INCL : bool = false > { data : V , separator : & 'p P , # [doc = " Marks the endpoint beyond which we've already found no separators."] tail : usize , }
};
}
