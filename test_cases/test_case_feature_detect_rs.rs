// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/support/feature_detect.rs
// Error: expected square brackets
// Problematic line: line 4

//! Helpers for runtime target feature detection that are shared across architectures.

// `AtomicU32` is preferred for a consistent size across targets.
#[cfg(all(target_has_atomic = "ptr", not(target_has_atomic = "32")))]
compile_error!("currently all targets that support `AtomicPtr` also support `AtomicU32`");

use core::sync::atomic::{AtomicU32, Ordering};
