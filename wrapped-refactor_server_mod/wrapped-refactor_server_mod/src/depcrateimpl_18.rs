// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl VisitMut for ItemRemover { fn visit_file_mut (& mut self , file : & mut File) { file . items . retain (| item | { match item { Item :: Const (item_const) => ! self . items_to_remove . contains (& item_const . ident . to_string ()) , Item :: Enum (item_enum) => ! self . items_to_remove . contains (& item_enum . ident . to_string ()) , Item :: Fn (item_fn) => ! self . items_to_remove . contains (& item_fn . sig . ident . to_string ()) , Item :: Struct (item_struct) => ! self . items_to_remove . contains (& item_struct . ident . to_string ()) , _ => true , } }) ; } }
};
}
