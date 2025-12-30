// Generated macro for f8_impl (function)
macro_rules! Depcrate_compressorf8_impl {
() => {
// Module: crate::compressor
// Provides: {"f8_impl"}
// Dependencies: {}
# [inline (always)] # [doc (hidden)] fn f8_impl < M : Machine > (mach : M , state : & mut [vec128_storage ; 8] , data : * const u8) { # [allow (clippy :: cast_ptr_alignment)] let data : * const M :: u128x1 = data . cast () ; let mut y = X8 :: < M > (mach . unpack (state [0]) , mach . unpack (state [1]) , mach . unpack (state [2]) , mach . unpack (state [3]) , mach . unpack (state [4]) , mach . unpack (state [5]) , mach . unpack (state [6]) , mach . unpack (state [7]) ,) ; unsafe { y . 0 ^= ptr :: read_unaligned (data) ; y . 1 ^= ptr :: read_unaligned (data . offset (1)) ; y . 2 ^= ptr :: read_unaligned (data . offset (2)) ; y . 3 ^= ptr :: read_unaligned (data . offset (3)) ; } for rc in crate :: consts :: E8_BITSLICE_ROUNDCONSTANT . chunks_exact (7) { unroll7 ! (j , { y = ss (y , unsafe { X2Bytes ::< M > { bytes : rc [j] } . x2 }) ; y = l (y) ; let f = match j { 0 => M :: u128x1 :: swap1 , 1 => M :: u128x1 :: swap2 , 2 => M :: u128x1 :: swap4 , 3 => M :: u128x1 :: swap8 , 4 => M :: u128x1 :: swap16 , 5 => M :: u128x1 :: swap32 , 6 => M :: u128x1 :: swap64 , _ => unreachable ! () , } ; y = X8 (y . 0 , f (y . 1) , y . 2 , f (y . 3) , y . 4 , f (y . 5) , y . 6 , f (y . 7)) ; }) ; } unsafe { y . 4 ^= ptr :: read_unaligned (data) ; y . 5 ^= ptr :: read_unaligned (data . offset (1)) ; y . 6 ^= ptr :: read_unaligned (data . offset (2)) ; y . 7 ^= ptr :: read_unaligned (data . offset (3)) ; } * state = [y . 0 . into () , y . 1 . into () , y . 2 . into () , y . 3 . into () , y . 4 . into () , y . 5 . into () , y . 6 . into () , y . 7 . into () ,] ; }
};
}
