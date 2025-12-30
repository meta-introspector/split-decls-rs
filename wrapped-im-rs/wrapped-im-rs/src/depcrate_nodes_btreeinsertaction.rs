// Generated macro for InsertAction (enum)
macro_rules! Depcrate_nodes_btreeInsertAction {
() => {
// Module: crate::nodes::btree
// Provides: {"InsertAction"}
// Dependencies: {}
enum InsertAction < A > { AddedAction , ReplacedAction (A) , InsertAt , InsertSplit (Node < A > , A , Node < A >) , }
};
}
