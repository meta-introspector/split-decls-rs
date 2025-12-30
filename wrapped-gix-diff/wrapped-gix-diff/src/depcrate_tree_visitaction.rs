// Generated macro for Action (enum)
macro_rules! Depcrate_tree_visitAction {
() => {
// Module: crate::tree::visit
// Provides: {"Action"}
// Dependencies: {}
# [doc = " What to do after a [Change] was [recorded](super::Visit::visit())."] # [derive (Default , Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the traversal of changes."] # [default] Continue , # [doc = " Stop the traversal of changes, making this the last call to [visit(…)](super::Visit::visit())."] Cancel , }
};
}
