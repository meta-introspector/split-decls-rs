// Generated macro for OpenStreamStats (struct)
macro_rules! Depcrate_statsOpenStreamStats {
() => {
// Module: crate::stats
// Provides: {"OpenStreamStats"}
// Dependencies: {}
# [doc = " Statistics for the currently open streams"] # [derive (Clone , Default)] pub struct OpenStreamStats (Arc < Mutex < Vec < Arc < StreamStats > > > >) ;
};
}
