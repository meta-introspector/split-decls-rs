// Generated macro for to_child (function)
macro_rules! Depcrate_tree_functionto_child {
() => {
// Module: crate::tree::function
// Provides: {"to_child"}
// Dependencies: {}
fn to_child (r : Option < Relation >) -> Option < Relation > { r . map (| r | match r { Relation :: Parent (id) => Relation :: ChildOfParent (id) , Relation :: ChildOfParent (id) => Relation :: ChildOfParent (id) , }) }
};
}
