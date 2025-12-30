// Generated macro for Compat01As03Sink (struct)
macro_rules! Depcrate_compat_compat01as03Compat01As03Sink {
() => {
// Module: crate::compat::compat01as03
// Provides: {"Compat01As03Sink"}
// Dependencies: {}
# [doc = " Converts a futures 0.1 Sink object to a futures 0.3-compatible version"] # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Compat01As03Sink < S , SinkItem > { pub (crate) inner : Spawn01 < S > , pub (crate) buffer : Option < SinkItem > , pub (crate) close_started : bool , }
};
}
