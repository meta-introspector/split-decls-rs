// Generated macro for gated (macro)
macro_rules! Depcrate_inert_attr_macrogated {
() => {
// Module: crate::inert_attr_macro
// Provides: {"gated"}
// Dependencies: {}
macro_rules ! gated { ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ?, $ gate : ident , $ msg : expr $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; ($ attr : ident , $ typ : expr , $ tpl : expr , $ duplicates : expr $ (, @ only_local : $ only_local : expr) ?, $ msg : expr $ (,) ?) => { BuiltinAttribute { name : stringify ! ($ attr) , template : $ tpl } } ; }
};
}
