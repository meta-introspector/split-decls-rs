use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types implementing this trait can be parsed from a positional numeral system with radix 10.
/// Acts much like `FromRadix10Signed`, but performs additional checks for overflows.
pub trait FromRadix10SignedChecked: FromRadix10Signed {
    /// Parses an integer from a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::FromRadix10SignedChecked;
    /// // Parsing to digits from a slice
    /// assert_eq!((Some(42),2), u32::from_radix_10_signed_checked(b"42"));
    /// // Additional bytes after the number are ignored
    /// assert_eq!((Some(42),2), u32::from_radix_10_signed_checked(b"42 is the answer to life, the universe and everything"));
    /// // (0,0) is returned if the slice does not start with a digit
    /// assert_eq!((Some(0),0), u32::from_radix_10_signed_checked(b"Sadly we do not know the question"));
    /// // While signed integer types are supported...
    /// assert_eq!((Some(42),2), i32::from_radix_10_signed_checked(b"42"));
    /// // Signs are allowed
    /// assert_eq!((Some(-42),3), i32::from_radix_10_signed_checked(b"-42"));
    /// // -0 is ok, even for an unsigned type
    /// assert_eq!((Some(0),2), u32::from_radix_10_signed_checked(b"-0"));
    /// // -1 is an Underflow
    /// assert_eq!((None,2), u32::from_radix_10_signed_checked(b"-1"));
    /// // Negative values for unsigned types are handled as `None`.
    /// assert_eq!((None,3), u32::from_radix_10_signed_checked(b"-42"));
    /// // Leading zeros are allowed
    /// assert_eq!((Some(42),4), u32::from_radix_10_signed_checked(b"0042"));
    /// // Overflow is indicated by `None`
    /// assert_eq!((None, 3), u8::from_radix_10_signed_checked(b"256"));
    /// assert_eq!((None, 4), i8::from_radix_10_signed_checked(b"+128"));
    /// assert_eq!((None, 4), i8::from_radix_10_signed_checked(b"-129"));
    /// ```
    ///
    /// # Return
    ///
    /// Returns a tuple with two numbers. The first is the integer parsed or zero if no digit has
    /// been found. None, if there were too many, or too high dighits and the parsing overflowed.
    /// The second is the index of the byte right after the parsed number. If the second element is
    /// zero the slice did not start with an ASCII digit.
    fn from_radix_10_signed_checked(_: &[u8]) -> (Option<Self>, usize);
}
