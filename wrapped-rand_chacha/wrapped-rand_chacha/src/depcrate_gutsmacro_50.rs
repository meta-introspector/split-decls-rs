// Generated macro for macro_50 (macro)
macro_rules! Depcrate_gutsmacro_50 {
() => {
// Module: crate::guts
// Provides: {"macro_50"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn init_chacha (key : & [u8 ; 32] , nonce : & [u8]) -> ChaCha { let ctr_nonce = [0 , if nonce . len () == 12 { read_u32le (& nonce [0 .. 4]) } else { 0 } , read_u32le (& nonce [nonce . len () - 8 .. nonce . len () - 4]) , read_u32le (& nonce [nonce . len () - 4 ..]) ,] ; let key0 : Mach :: u32x4 = m . read_le (& key [.. 16]) ; let key1 : Mach :: u32x4 = m . read_le (& key [16 ..]) ; ChaCha { b : key0 . into () , c : key1 . into () , d : ctr_nonce . into () , } } }) ;
};
}
