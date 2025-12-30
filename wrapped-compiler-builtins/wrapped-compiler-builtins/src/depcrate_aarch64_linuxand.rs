// Generated macro for and (macro)
macro_rules! Depcrate_aarch64_linuxand {
() => {
// Module: crate::aarch64_linux
// Provides: {"and"}
// Dependencies: {}
macro_rules ! and { ($ ordering : ident , $ bytes : tt , $ name : ident) => { fetch_op ! { $ ordering , $ bytes , $ name , "bic" , "ldclr" } } ; }
};
}
