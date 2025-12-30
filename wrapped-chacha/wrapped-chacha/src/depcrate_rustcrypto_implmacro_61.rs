// Generated macro for macro_61 (macro)
macro_rules! Depcrate_rustcrypto_implmacro_61 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"macro_61"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn init_chacha (key : & GenericArray < u8 , U32 >, nonce : & [u8]) -> ChaCha { let ctr_nonce = [0 , if nonce . len () == 12 { u32 :: from_le_bytes (nonce [0 .. 4] . try_into () . unwrap ()) } else { 0 } , u32 :: from_le_bytes (nonce [nonce . len () - 8 .. nonce . len () - 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (nonce [nonce . len () - 4 ..] . try_into () . unwrap ()) ,] ; let key0 : Mach :: u32x4 = m . read_le (& key [.. 16]) ; let key1 : Mach :: u32x4 = m . read_le (& key [16 ..]) ; ChaCha { b : key0 . into () , c : key1 . into () , d : ctr_nonce . into () , } } }) ;
};
}
