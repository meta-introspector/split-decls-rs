// Generated macro for State (struct)
macro_rules! Depcrate_treeState {
() => {
// Module: crate::tree
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state required to run [tree-diffs](super::tree())."] # [derive (Default , Clone)] pub struct State { # [doc = " A buffer for object data."] pub buf1 : Vec < u8 > , # [doc = " Another buffer for object data."] pub buf2 : Vec < u8 > , trees : VecDeque < TreeInfoTuple > , change_id : visit :: ChangeId , }
};
}
