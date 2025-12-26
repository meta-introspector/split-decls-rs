use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compares two 128-bit byte strings in constant time.
///
/// # Examples
///
/// ```
/// use constant_time_eq::constant_time_eq_16;
///
/// assert!(constant_time_eq_16(&[3; 16], &[3; 16]));
/// assert!(!constant_time_eq_16(&[3; 16], &[7; 16]));
/// ```
#[inline]
#[must_use]
pub fn constant_time_eq_16(a: &[u8; 16], b: &[u8; 16]) -> bool {
    constant_time_eq_n(a, b)
}
