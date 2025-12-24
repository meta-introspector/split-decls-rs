use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    all(
        feature = "simd-accel",
        any(
            target_feature = "sse2",
            all(target_endian = "little", target_arch = "aarch64"),
            all(target_endian = "little", target_feature = "neon")
        )
    )
)]
mod simd_funcs;
