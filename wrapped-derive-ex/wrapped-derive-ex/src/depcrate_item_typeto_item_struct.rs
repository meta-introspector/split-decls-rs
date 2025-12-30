// Generated macro for to_item_struct (function)
macro_rules! Depcrate_item_typeto_item_struct {
() => {
// Module: crate::item_type
// Provides: {"to_item_struct"}
// Dependencies: {}
fn to_item_struct (item : & DeriveInput , data : & DataStruct) -> ItemStruct { ItemStruct { attrs : item . attrs . clone () , vis : item . vis . clone () , struct_token : data . struct_token , ident : item . ident . clone () , generics : item . generics . clone () , fields : data . fields . clone () , semi_token : data . semi_token , } }
};
}
