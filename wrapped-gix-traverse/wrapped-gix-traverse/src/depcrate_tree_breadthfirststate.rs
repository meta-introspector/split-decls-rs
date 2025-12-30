// Generated macro for State (struct)
macro_rules! Depcrate_tree_breadthfirstState {
() => {
// Module: crate::tree::breadthfirst
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state used and potentially shared by multiple tree traversals."] # [derive (Default , Clone)] pub struct State { next : VecDeque < ObjectId > , buf : Vec < u8 > , }
};
}
