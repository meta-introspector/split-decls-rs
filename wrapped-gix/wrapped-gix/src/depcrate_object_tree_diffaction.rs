// Generated macro for Action (enum)
macro_rules! Depcrate_object_tree_diffAction {
() => {
// Module: crate::object::tree::diff
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Returned by the `for_each` function to control flow."] # [derive (Default , Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the traversal of changes."] # [default] Continue , # [doc = " Stop the traversal of changes and stop calling this function."] Cancel , }
};
}
