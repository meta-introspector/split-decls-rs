// Generated macro for macro_28 (macro)
macro_rules! Depcrate_gutsmacro_28 {
() => {
// Module: crate::guts
// Provides: {"macro_28"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn init_chacha_x (key : & [u8 ; 32] , nonce : & [u8 ; 24] , rounds : u32) -> ChaCha { let key0 : Mach :: u32x4 = m . read_le (& key [.. 16]) ; let key1 : Mach :: u32x4 = m . read_le (& key [16 ..]) ; let nonce0 : Mach :: u32x4 = m . read_le (& nonce [.. 16]) ; let mut state = ChaCha { b : key0 . into () , c : key1 . into () , d : nonce0 . into () , } ; let x = refill_narrow_rounds (& mut state , rounds) ; let ctr_nonce1 = [0 , 0 , read_u32le (& nonce [16 .. 20]) , read_u32le (& nonce [20 .. 24])] ; state . b = x . a ; state . c = x . d ; state . d = ctr_nonce1 . into () ; state } }) ;
};
}
