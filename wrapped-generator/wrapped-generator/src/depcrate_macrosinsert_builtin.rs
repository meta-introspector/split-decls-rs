// Generated macro for insert_builtin (macro)
macro_rules! Depcrate_macrosinsert_builtin {
() => {
// Module: crate::macros
// Provides: {"insert_builtin"}
// Dependencies: {}
macro_rules ! insert_builtin { ($ builtin : expr , $ name : ident , $ pattern : expr) => { $ builtin . push ((stringify ! ($ name) , generate_rule ! ($ name , $ pattern))) ; } ; }
};
}
