// Generated macro for macro_48 (macro)
macro_rules! Depcrate_gutsmacro_48 {
() => {
// Module: crate::guts
// Provides: {"macro_48"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn get_seed (state : & ChaCha) -> [u8 ; 32] { let b : Mach :: u32x4 = m . unpack (state . b) ; let c : Mach :: u32x4 = m . unpack (state . c) ; let mut key = [0u8 ; 32] ; b . write_le (& mut key [.. 16]) ; c . write_le (& mut key [16 ..]) ; key } }) ;
};
}
