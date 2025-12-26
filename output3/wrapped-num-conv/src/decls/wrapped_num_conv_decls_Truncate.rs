use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Truncate to an integer of the same size or smaller, preserving the least significant bits.
///
/// ```rust
/// # use num_conv::Truncate;
/// assert_eq!(u16::MAX.truncate::<u8>(), u8::MAX);
/// assert_eq!(u32::MAX.truncate::<u16>(), u16::MAX);
/// assert_eq!(u64::MAX.truncate::<u32>(), u32::MAX);
/// assert_eq!(u128::MAX.truncate::<u64>(), u64::MAX);
/// ```
///
/// ```rust
/// # use num_conv::Truncate;
/// assert_eq!((-1_i16).truncate::<i8>(), -1_i8);
/// assert_eq!((-1_i32).truncate::<i16>(), -1_i16);
/// assert_eq!((-1_i64).truncate::<i32>(), -1_i32);
/// assert_eq!((-1_i128).truncate::<i64>(), -1_i64);
/// ```
pub trait Truncate: sealed::Integer {
    /// Truncate an integer to an integer of the same size or smaller, preserving the least
    /// significant bits.
    fn truncate<T>(self) -> T
    where
        Self: TruncateTarget<T>;
}
