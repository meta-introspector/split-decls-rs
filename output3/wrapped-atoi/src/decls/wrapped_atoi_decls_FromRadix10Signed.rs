use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types implementing this trait can be parsed from a positional numeral system with radix 10. This
/// trait allows for an additional sign character (`+` or `-`) in front of the actual number in
/// order, to allow for parsing negative values.
pub trait FromRadix10Signed: Sized {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10Signed;
    /// // Parsing to digits from a slice
    /// assert_eq!((42,2), i32::from_radix_10_signed(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((42,2), i32::from_radix_10_signed(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((0,0), i32::from_radix_10_signed(b"Sadly we do not know the question"));
    /// // Signs are allowed
    /// assert_eq!((-42,3), i32::from_radix_10_signed(b"-42"));
    /// // Signs are allowed
    /// assert_eq!((42,3), i32::from_radix_10_signed(b"+42"));
    /// // Even on unsigned types.
    /// assert_eq!((0,2), u32::from_radix_10_signed(b"-0"));
    /// // Leading zeros are allowed
    /// assert_eq!((42,4), i32::from_radix_10_signed(b"0042"));
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero, the second is the
    /// index of the byte right after the parsed number. If the second element is zero the slice
    /// did not start with an ASCII digit.
    fn from_radix_10_signed(_: &[u8]) -> (Self, usize);
}
