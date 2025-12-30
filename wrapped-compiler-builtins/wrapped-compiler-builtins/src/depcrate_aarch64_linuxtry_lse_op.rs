// Generated macro for try_lse_op (macro)
macro_rules! Depcrate_aarch64_linuxtry_lse_op {
() => {
// Module: crate::aarch64_linux
// Provides: {"try_lse_op"}
// Dependencies: {}
macro_rules ! try_lse_op { ($ op : literal , $ ordering : ident , $ bytes : tt , $ ($ reg : literal ,) * [$ mem : ident]) => { concat ! (".arch_extension lse; " , "adrp    x16, {have_lse}; " , "ldrb    w16, [x16, :lo12:{have_lse}]; " , "cbz     w16, 8f; " , concat ! (lse ! ($ op , $ ordering , $ bytes) , $ (" " , reg ! ($ bytes , $ reg) , ", " ,) * "[" , stringify ! ($ mem) , "]; " ,) , "ret; " , "8:") } ; }
};
}
