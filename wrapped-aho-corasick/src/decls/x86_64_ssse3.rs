macro_rules! deps {
    () => {
        I32!();
        Vector!();
        I8!();
    };
}

macro_rules! x86_64_ssse3 {
    () => {
        deps!();
        # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] mod x86_64_ssse3 { use core :: arch :: x86_64 :: * ; use crate :: util :: int :: { I32 , I8 } ; use super :: Vector ; impl Vector for __m128i { const BITS : usize = 128 ; const BYTES : usize = 16 ; # [inline (always)] unsafe fn splat (byte : u8) -> __m128i { _mm_set1_epi8 (i8 :: from_bits (byte)) } # [inline (always)] unsafe fn load_unaligned (data : * const u8) -> __m128i { _mm_loadu_si128 (data . cast :: < __m128i > ()) } # [inline (always)] unsafe fn is_zero (self) -> bool { let cmp = self . cmpeq (Self :: splat (0)) ; _mm_movemask_epi8 (cmp) . to_bits () == 0xFFFF } # [inline (always)] unsafe fn cmpeq (self , vector2 : Self) -> __m128i { _mm_cmpeq_epi8 (self , vector2) } # [inline (always)] unsafe fn and (self , vector2 : Self) -> __m128i { _mm_and_si128 (self , vector2) } # [inline (always)] unsafe fn or (self , vector2 : Self) -> __m128i { _mm_or_si128 (self , vector2) } # [inline (always)] unsafe fn shift_8bit_lane_right < const BITS : i32 > (self) -> Self { let lomask = Self :: splat (0xF) ; _mm_srli_epi16 (self , BITS) . and (lomask) } # [inline (always)] unsafe fn shift_in_one_byte (self , vector2 : Self) -> Self { _mm_alignr_epi8 (self , vector2 , 15) } # [inline (always)] unsafe fn shift_in_two_bytes (self , vector2 : Self) -> Self { _mm_alignr_epi8 (self , vector2 , 14) } # [inline (always)] unsafe fn shift_in_three_bytes (self , vector2 : Self) -> Self { _mm_alignr_epi8 (self , vector2 , 13) } # [inline (always)] unsafe fn shuffle_bytes (self , indices : Self) -> Self { _mm_shuffle_epi8 (self , indices) } # [inline (always)] unsafe fn for_each_64bit_lane < T > (self , mut f : impl FnMut (usize , u64) -> Option < T > ,) -> Option < T > { let lanes : [u64 ; 2] = core :: mem :: transmute (self) ; if let Some (t) = f (0 , lanes [0]) { return Some (t) ; } if let Some (t) = f (1 , lanes [1]) { return Some (t) ; } None } } }
    };
}

x86_64_ssse3!()