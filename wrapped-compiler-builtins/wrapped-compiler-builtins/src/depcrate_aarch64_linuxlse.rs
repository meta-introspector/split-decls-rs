// Generated macro for lse (macro)
macro_rules! Depcrate_aarch64_linuxlse {
() => {
// Module: crate::aarch64_linux
// Provides: {"lse"}
// Dependencies: {}
macro_rules ! lse { ($ op : literal , $ order : ident , 16) => { concat ! ($ op , "p" , lse_mem_sfx ! ($ order)) } ; ($ op : literal , $ order : ident , $ bytes : tt) => { concat ! ($ op , lse_mem_sfx ! ($ order) , size ! ($ bytes)) } ; }
};
}
