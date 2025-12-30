// Generated macro for Cache (struct)
macro_rules! Depcrate_backtrackCache {
() => {
// Module: crate::backtrack
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " Shared cached state between multiple invocations of a backtracking engine"] # [doc = " in the same thread."] # [derive (Clone , Debug)] pub struct Cache { jobs : Vec < Job > , visited : Vec < Bits > , }
};
}
