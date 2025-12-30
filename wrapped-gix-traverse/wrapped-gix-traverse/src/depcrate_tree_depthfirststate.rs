// Generated macro for State (struct)
macro_rules! Depcrate_tree_depthfirstState {
() => {
// Module: crate::tree::depthfirst
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state used and potentially shared by multiple tree traversals, reusing memory."] # [derive (Default , Clone)] pub struct State { freelist : Vec < Vec < u8 > > , }
};
}
