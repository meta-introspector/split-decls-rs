use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Constant-time equality check for two byte slices.
///
/// # Arguments
///
/// * `x` - First byte slice
/// * `y` - Second byte slice
///
/// # Returns
///
/// * `bool` - `true` if the slices are equal, `false` otherwise
pub fn verify(x: &[u8], y: &[u8]) -> bool {
    if x.len() != y.len() {
        return false;
    }
    let mut v: u32 = 0;
    let (mut h1, mut h2) = (0u32, 0u32);
    for (b1, b2) in x.iter().zip(y.iter()) {
        h1 ^= (h1 << 5).wrapping_add((h1 >> 2) ^ *b1 as u32);
        h2 ^= (h2 << 5).wrapping_add((h2 >> 2) ^ *b2 as u32);
    }
    v |= h1 ^ h2;
    for (a, b) in x.iter().zip(y.iter()) {
        v |= (a ^ b) as u32;
    }
    v == 0
}
