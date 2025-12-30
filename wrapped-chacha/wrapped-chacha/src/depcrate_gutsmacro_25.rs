// Generated macro for macro_25 (macro)
macro_rules! Depcrate_gutsmacro_25 {
() => {
// Module: crate::guts
// Provides: {"macro_25"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn refill_narrow (state : & mut ChaCha , drounds : u32 , out : & mut [u8 ; BLOCK]) { let x = refill_narrow_rounds (state , drounds) ; let x = State { a : m . unpack (x . a) , b : m . unpack (x . b) , c : m . unpack (x . c) , d : m . unpack (x . d) , } ; state . output_narrow (m , x , out) ; state . inc_block_ct (m) ; } }) ;
};
}
