// Generated macro for refill_wide_impl (function)
macro_rules! Depcrate_gutsrefill_wide_impl {
() => {
// Module: crate::guts
// Provides: {"refill_wide_impl"}
// Dependencies: {}
# [allow (clippy :: many_single_char_names)] # [inline (always)] fn refill_wide_impl < Mach : Machine > (m : Mach , state : & mut ChaCha , drounds : u32 , out : & mut [u8 ; BUFSZ] ,) { let k = m . vec ([0x6170_7865 , 0x3320_646e , 0x7962_2d32 , 0x6b20_6574]) ; let b = m . unpack (state . b) ; let c = m . unpack (state . c) ; let mut x = State { a : Mach :: u32x4x4 :: from_lanes ([k , k , k , k]) , b : Mach :: u32x4x4 :: from_lanes ([b , b , b , b]) , c : Mach :: u32x4x4 :: from_lanes ([c , c , c , c]) , d : d0123 (m , state . d) , } ; for _ in 0 .. drounds { x = round (x) ; x = undiagonalize (round (diagonalize (x))) ; } let kk = Mach :: u32x4x4 :: from_lanes ([k , k , k , k]) ; let sb = m . unpack (state . b) ; let sb = Mach :: u32x4x4 :: from_lanes ([sb , sb , sb , sb]) ; let sc = m . unpack (state . c) ; let sc = Mach :: u32x4x4 :: from_lanes ([sc , sc , sc , sc]) ; let sd = d0123 (m , state . d) ; let results = Mach :: u32x4x4 :: transpose4 (x . a + kk , x . b + sb , x . c + sc , x . d + sd) ; results . 0 . write_le (& mut out [0 .. 64]) ; results . 1 . write_le (& mut out [64 .. 128]) ; results . 2 . write_le (& mut out [128 .. 192]) ; results . 3 . write_le (& mut out [192 .. 256]) ; state . d = add_pos (m , sd . to_lanes () [0] , 4) . into () ; }
};
}
