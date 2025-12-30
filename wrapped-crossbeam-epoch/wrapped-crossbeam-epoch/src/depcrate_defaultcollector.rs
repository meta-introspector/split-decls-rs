// Generated macro for collector (function)
macro_rules! Depcrate_defaultcollector {
() => {
// Module: crate::default
// Provides: {"collector"}
// Dependencies: {}
fn collector () -> & 'static Collector { # [cfg (not (crossbeam_loom))] { # [doc = " The global data for the default garbage collector."] static COLLECTOR : OnceLock < Collector > = OnceLock :: new () ; COLLECTOR . get_or_init (Collector :: new) } # [cfg (crossbeam_loom)] { loom :: lazy_static ! { # [doc = " The global data for the default garbage collector."] static ref COLLECTOR : Collector = Collector :: new () ; } & COLLECTOR } }
};
}
