// Generated macro for Config (struct)
macro_rules! Depcrate_nfa_thompson_backtrackConfig {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration used for building a bounded backtracker."] # [doc = ""] # [doc = " A bounded backtracker configuration is a simple data object that is"] # [doc = " typically used with [`Builder::configure`]."] # [derive (Clone , Debug , Default)] pub struct Config { pre : Option < Option < Prefilter > > , visited_capacity : Option < usize > , }
};
}
