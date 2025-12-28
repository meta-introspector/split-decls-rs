macro_rules! macro_62 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { cfg_if :: cfg_if ! { if # [cfg (blake3_avx512_ffi)] { pub const MAX_SIMD_DEGREE_OR_2 : usize = 16 ; } else { pub const MAX_SIMD_DEGREE_OR_2 : usize = 8 ; } } } else if # [cfg (blake3_neon)] { pub const MAX_SIMD_DEGREE_OR_2 : usize = 4 ; } else if # [cfg (blake3_wasm32_simd)] { pub const MAX_SIMD_DEGREE_OR_2 : usize = 4 ; } else { pub const MAX_SIMD_DEGREE_OR_2 : usize = 2 ; } }
    };
}

macro_62!()