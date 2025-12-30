// Generated macro for is_proc_macro (function)
macro_rules! Depcrateis_proc_macro {
() => {
// Module: crate
// Provides: {"is_proc_macro"}
// Dependencies: {}
fn is_proc_macro (attrs : & [Attribute]) -> bool { attrs . iter () . any (| attr | { attr . path_is_ident ("proc_macro") || attr . path_is_ident ("proc_macro_derive") || attr . path_is_ident ("proc_macro_attribute") }) }
};
}
