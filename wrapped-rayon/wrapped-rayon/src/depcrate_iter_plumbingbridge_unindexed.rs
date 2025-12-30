// Generated macro for bridge_unindexed (function)
macro_rules! Depcrate_iter_plumbingbridge_unindexed {
() => {
// Module: crate::iter::plumbing
// Provides: {"bridge_unindexed"}
// Dependencies: {}
# [doc = " A variant of [`bridge_producer_consumer()`] where the producer is an unindexed producer."] pub fn bridge_unindexed < P , C > (producer : P , consumer : C) -> C :: Result where P : UnindexedProducer , C : UnindexedConsumer < P :: Item > , { let splitter = Splitter :: new () ; bridge_unindexed_producer_consumer (false , splitter , producer , consumer) }
};
}
