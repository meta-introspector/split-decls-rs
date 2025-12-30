// Generated macro for SkipList (struct)
macro_rules! Depcrate_baseSkipList {
() => {
// Module: crate::base
// Provides: {"SkipList"}
// Dependencies: {}
# [doc = " A lock-free skip list."] pub struct SkipList < K , V > { # [doc = " The head of the skip list (just a dummy node, not a real entry)."] head : Head < K , V > , # [doc = " The `Collector` associated with this skip list."] collector : Collector , # [doc = " Hot data associated with the skip list, stored in a dedicated cache line."] hot_data : CachePadded < HotData > , }
};
}
