// Generated macro for InsertionPoint (enum)
macro_rules! Depcrate_tree_builder_typesInsertionPoint {
() => {
// Module: crate::tree_builder::types
// Provides: {"InsertionPoint"}
// Dependencies: {}
pub (crate) enum InsertionPoint < Handle > { # [doc = " Insert as last child in this parent."] LastChild (Handle) , # [allow (dead_code)] # [doc = " Insert before this following sibling."] BeforeSibling (Handle) , # [doc = " Insertion point is decided based on existence of element's parent node."] TableFosterParenting { element : Handle , prev_element : Handle , } , }
};
}
