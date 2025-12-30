// Generated macro for macro_46 (macro)
macro_rules! Depcrate_rustcrypto_implmacro_46 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"macro_46"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn seek32 (buf : & mut Buffer , ct : u64) { let blockct = ct / BLOCK64 ; assert ! (blockct < SMALL_LEN || (blockct == SMALL_LEN && ct % BLOCK64 == 0)) ; buf . len = SMALL_LEN - blockct ; buf . have = - ((ct % BLOCK64) as i8) ; buf . state . seek32 (m , blockct as u32) ; } }) ;
};
}
