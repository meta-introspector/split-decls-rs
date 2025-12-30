// Generated macro for impl_16 (impl)
macro_rules! Depcrate_tree_itemimpl_16 {
() => {
// Module: crate::tree::item
// Provides: {"impl_16"}
// Dependencies: {}
impl crate :: Progress for Item { fn init (& mut self , max : Option < Step > , unit : Option < Unit >) { Item :: init (self , max , unit) } fn unit (& self) -> Option < Unit > { Item :: unit (self) } fn max (& self) -> Option < usize > { Item :: max (self) } fn set_max (& mut self , max : Option < Step >) -> Option < Step > { Item :: set_max (self , max) } fn set_name (& mut self , name : String) { Item :: set_name (self , name) } fn name (& self) -> Option < String > { Item :: name (self) } fn id (& self) -> Id { Item :: id (self) } fn message (& self , level : MessageLevel , message : String) { Item :: message (self , level , message) } }
};
}
