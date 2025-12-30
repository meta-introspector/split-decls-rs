// Generated macro for use_82 (use)
macro_rules! Depcrate_collectoruse_82 {
() => {
// Module: crate::collector
// Provides: {"use_82"}
// Dependencies: {}
# [doc = " Epoch-based garbage collector."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Collector;"] # [doc = ""] # [doc = " let collector = Collector::new();"] # [doc = ""] # [doc = " let handle = collector.register();"] # [doc = " drop(collector); // `handle` still works after dropping `collector`"] # [doc = ""] # [doc = " handle.pin().flush();"] # [doc = " ```"] use core :: fmt ;
};
}
