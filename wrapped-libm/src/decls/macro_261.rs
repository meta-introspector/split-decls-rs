macro_rules! macro_261 {
    () => {
        # [cfg (arch_enabled)] cfg_if ! { if # [cfg (all (target_arch = "wasm32" , intrinsics_enabled))] { mod wasm32 ; pub use wasm32 :: { ceil , ceilf , fabs , fabsf , floor , floorf , rint , rintf , sqrt , sqrtf , trunc , truncf , } ; } else if # [cfg (target_feature = "sse2")] { mod x86 ; pub use x86 :: { sqrt , sqrtf , fma , fmaf } ; } else if # [cfg (all (any (target_arch = "aarch64" , target_arch = "arm64ec") , target_feature = "neon"))] { mod aarch64 ; pub use aarch64 :: { fma , fmaf , rint , rintf , sqrt , sqrtf , } ; # [cfg (all (f16_enabled , target_feature = "fp16"))] pub use aarch64 :: { rintf16 , sqrtf16 , } ; } }
    };
}

macro_261!();