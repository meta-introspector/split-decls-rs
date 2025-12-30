// Generated macro for or (macro)
macro_rules! Depcrate_aarch64_linuxor {
() => {
// Module: crate::aarch64_linux
// Provides: {"or"}
// Dependencies: {}
macro_rules ! or { ($ ordering : ident , $ bytes : tt , $ name : ident) => { fetch_op ! { $ ordering , $ bytes , $ name , "orr" , "ldset" } } ; }
};
}
