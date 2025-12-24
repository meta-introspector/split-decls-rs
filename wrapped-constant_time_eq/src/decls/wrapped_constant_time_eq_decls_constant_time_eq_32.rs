use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compares two 256-bit byte strings in constant time.
///
/// # Examples
///
/// ```
/// use constant_time_eq::constant_time_eq_32;
///
/// assert!(constant_time_eq_32(&[3; 32], &[3; 32]));
/// assert!(!constant_time_eq_32(&[3; 32], &[7; 32]));
/// ```
#[inline]
#[must_use]
pub fn constant_time_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    constant_time_eq_n(a, b)
}
