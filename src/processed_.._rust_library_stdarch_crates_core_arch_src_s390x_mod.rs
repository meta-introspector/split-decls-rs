// SRC: ../rust/library/stdarch/crates/core_arch/src/s390x/mod.rs
//! `s390x` intrinsics

pub(crate) mod macros;

mod vector;
#[unstable(feature = "stdarch_s390x", issue = "130869")]
pub use self::vector::*;
