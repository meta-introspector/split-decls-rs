macro_rules! simd_funcs {
    () => {
        # [cfg (all (feature = "simd-accel" , any (target_feature = "sse2" , all (target_endian = "little" , target_arch = "aarch64") , all (target_endian = "little" , target_feature = "neon"))))] mod simd_funcs ;
    };
}

simd_funcs!()