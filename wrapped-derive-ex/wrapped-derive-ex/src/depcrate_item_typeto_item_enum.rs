// Generated macro for to_item_enum (function)
macro_rules! Depcrate_item_typeto_item_enum {
() => {
// Module: crate::item_type
// Provides: {"to_item_enum"}
// Dependencies: {}
fn to_item_enum (item : & DeriveInput , data : & DataEnum) -> ItemEnum { ItemEnum { attrs : item . attrs . clone () , vis : item . vis . clone () , enum_token : data . enum_token , ident : item . ident . clone () , generics : item . generics . clone () , brace_token : data . brace_token , variants : data . variants . clone () , } }
};
}
