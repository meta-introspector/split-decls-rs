use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compares two 512-bit byte strings in constant time.
///
/// # Examples
///
/// ```
/// use constant_time_eq::constant_time_eq_64;
///
/// assert!(constant_time_eq_64(&[3; 64], &[3; 64]));
/// assert!(!constant_time_eq_64(&[3; 64], &[7; 64]));
/// ```
#[inline]
#[must_use]
pub fn constant_time_eq_64(a: &[u8; 64], b: &[u8; 64]) -> bool {
    constant_time_eq_n(a, b)
}
