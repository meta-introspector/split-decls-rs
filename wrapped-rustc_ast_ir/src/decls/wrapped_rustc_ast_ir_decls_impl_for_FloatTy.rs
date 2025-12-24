use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FloatTy {
    pub fn name_str(self) -> &'static str {
        match self {
            FloatTy::F16 => "f16",
            FloatTy::F32 => "f32",
            FloatTy::F64 => "f64",
            FloatTy::F128 => "f128",
        }
    }
    #[cfg(feature = "nightly")]
    pub fn name(self) -> Symbol {
        match self {
            FloatTy::F16 => sym::f16,
            FloatTy::F32 => sym::f32,
            FloatTy::F64 => sym::f64,
            FloatTy::F128 => sym::f128,
        }
    }
    pub fn bit_width(self) -> u64 {
        match self {
            FloatTy::F16 => 16,
            FloatTy::F32 => 32,
            FloatTy::F64 => 64,
            FloatTy::F128 => 128,
        }
    }
}
