// Generated macro for macro_45 (macro)
macro_rules! Depcrate_rustcrypto_implmacro_45 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"macro_45"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn seek64 (buf : & mut Buffer , ct : u64) { let blockct = ct / BLOCK64 ; buf . len = BIG_LEN . wrapping_sub (blockct) ; buf . fresh = blockct == 0 ; buf . have = - ((ct % BLOCK64) as i8) ; buf . state . seek64 (m , blockct) ; } }) ;
};
}
