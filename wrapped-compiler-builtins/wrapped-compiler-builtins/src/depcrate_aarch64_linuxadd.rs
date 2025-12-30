// Generated macro for add (macro)
macro_rules! Depcrate_aarch64_linuxadd {
() => {
// Module: crate::aarch64_linux
// Provides: {"add"}
// Dependencies: {}
macro_rules ! add { ($ ordering : ident , $ bytes : tt , $ name : ident) => { fetch_op ! { $ ordering , $ bytes , $ name , "add" , "ldadd" } } ; }
};
}
