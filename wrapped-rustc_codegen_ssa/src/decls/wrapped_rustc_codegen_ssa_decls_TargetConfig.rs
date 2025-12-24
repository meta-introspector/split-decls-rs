use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Target-specific options that get set in `cfg(...)`.
///
/// RUSTC_SPECIFIC_FEATURES should be skipped here, those are handled outside codegen.
pub struct TargetConfig {
    /// Options to be set in `cfg(target_features)`.
    pub target_features: Vec<Symbol>,
    /// Options to be set in `cfg(target_features)`, but including unstable features.
    pub unstable_target_features: Vec<Symbol>,
    /// Option for `cfg(target_has_reliable_f16)`, true if `f16` basic arithmetic works.
    pub has_reliable_f16: bool,
    /// Option for `cfg(target_has_reliable_f16_math)`, true if `f16` math calls work.
    pub has_reliable_f16_math: bool,
    /// Option for `cfg(target_has_reliable_f128)`, true if `f128` basic arithmetic works.
    pub has_reliable_f128: bool,
    /// Option for `cfg(target_has_reliable_f128_math)`, true if `f128` math calls work.
    pub has_reliable_f128_math: bool,
}
