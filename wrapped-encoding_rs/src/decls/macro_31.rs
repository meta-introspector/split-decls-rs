macro_rules! macro_31 {
    () => {
        cfg_if ! { if # [cfg (all (target_feature = "sse2" , target_arch = "x86_64"))] { use core :: arch :: x86_64 :: __m128i ; use core :: arch :: x86_64 :: _mm_movemask_epi8 ; use core :: arch :: x86_64 :: _mm_packus_epi16 ; } else if # [cfg (all (target_feature = "sse2" , target_arch = "x86"))] { use core :: arch :: x86 :: __m128i ; use core :: arch :: x86 :: _mm_movemask_epi8 ; use core :: arch :: x86 :: _mm_packus_epi16 ; } else if # [cfg (target_arch = "aarch64")] { use core :: arch :: aarch64 :: vmaxvq_u8 ; use core :: arch :: aarch64 :: vmaxvq_u16 ; } else { } }
    };
}

macro_31!();