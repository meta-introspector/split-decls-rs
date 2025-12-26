use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types implementing this trait can be parsed from a positional numeral system with radix 16.
/// Acts much like `FromRadix16`, but performs additional checks for overflows.
pub trait FromRadix16Checked: FromRadix16 {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix16Checked;
    /// // Parsing to digits from a slice
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2a"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2a is the answer to life, the \
    /// universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((Some(0),0), u32::from_radix_16_checked(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((Some(42),2), i32::from_radix_16_checked(b"2a"));
    /// // Signs are not allowed (even for signed integer types)
    /// assert_eq!((Some(0),0), i32::from_radix_16_checked(b"-2a"));
    /// // Leading zeros are allowed
    /// assert_eq!((Some(42),4), u32::from_radix_16_checked(b"002a"));
    /// // So are uppercase letters
    /// assert_eq!((Some(42),2), u32::from_radix_16_checked(b"2A"))
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero if no digit has
    /// been found. None, if there were too many, or too high dighits and the parsing overflowed.
    /// The second is the index of the byte right after the parsed number. If the second element is
    /// zero the slice did not start with an ASCII digit.
    fn from_radix_16_checked(_: &[u8]) -> (Option<Self>, usize);
}
