// Generated macro for Action (enum)
macro_rules! Depcrate_tree_with_rewritesAction {
() => {
// Module: crate::tree_with_rewrites
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Returned by the [`tree_with_rewrites()`](super::tree_with_rewrites()) function to control flow."] # [derive (Default , Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the traversal of changes."] # [default] Continue , # [doc = " Stop the traversal of changes and stop calling the function that returned it."] Cancel , }
};
}
