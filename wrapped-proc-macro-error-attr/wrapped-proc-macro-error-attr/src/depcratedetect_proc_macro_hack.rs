// Generated macro for detect_proc_macro_hack (function)
macro_rules! Depcratedetect_proc_macro_hack {
() => {
// Module: crate
// Provides: {"detect_proc_macro_hack"}
// Dependencies: {}
fn detect_proc_macro_hack (attrs : & [Attribute]) -> bool { attrs . iter () . any (| attr | attr . path_is_ident ("proc_macro_hack")) }
};
}
