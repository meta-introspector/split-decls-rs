// Generated macro for impl_17 (impl)
macro_rules! Depcrate_tree_itemimpl_17 {
() => {
// Module: crate::tree::item
// Provides: {"impl_17"}
// Dependencies: {}
impl crate :: NestedProgress for Item { type SubProgress = Item ; fn add_child (& mut self , name : impl Into < String >) -> Self { Item :: add_child (self , name) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self { Item :: add_child_with_id (self , name , id) } }
};
}
