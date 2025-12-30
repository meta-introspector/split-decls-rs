// Generated macro for xor (macro)
macro_rules! Depcrate_aarch64_linuxxor {
() => {
// Module: crate::aarch64_linux
// Provides: {"xor"}
// Dependencies: {}
macro_rules ! xor { ($ ordering : ident , $ bytes : tt , $ name : ident) => { fetch_op ! { $ ordering , $ bytes , $ name , "eor" , "ldeor" } } ; }
};
}
