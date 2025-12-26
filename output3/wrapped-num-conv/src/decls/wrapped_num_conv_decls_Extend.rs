use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extend to an integer of the same size or larger, preserving its value.
///
/// ```rust
/// # use num_conv::Extend;
/// assert_eq!(0_u8.extend::<u16>(), 0_u16);
/// assert_eq!(0_u16.extend::<u32>(), 0_u32);
/// assert_eq!(0_u32.extend::<u64>(), 0_u64);
/// assert_eq!(0_u64.extend::<u128>(), 0_u128);
/// ```
///
/// ```rust
/// # use num_conv::Extend;
/// assert_eq!((-1_i8).extend::<i16>(), -1_i16);
/// assert_eq!((-1_i16).extend::<i32>(), -1_i32);
/// assert_eq!((-1_i32).extend::<i64>(), -1_i64);
/// assert_eq!((-1_i64).extend::<i128>(), -1_i128);
/// ```
pub trait Extend: sealed::Integer {
    /// Extend an integer to an integer of the same size or larger, preserving its value.
    fn extend<T>(self) -> T
    where
        Self: ExtendTarget<T>;
}
