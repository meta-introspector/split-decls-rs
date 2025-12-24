use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    all(
        target_arch = "x86_64",
        target_feature = "sse2",
        not(feature = "no_simd"),
        not(miri)
    )
)]
pub mod x86_64;
