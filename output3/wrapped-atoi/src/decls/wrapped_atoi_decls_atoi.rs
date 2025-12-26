use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parses an integer from a slice.
///
/// Contrary to its 'C' counterpart atoi is generic and will require a type argument if the type
/// inference can not determine its result. It will also check for overflow / underflow and allow
/// for Signs.
///
/// Use [`FromRadix10`] or [`FromRadix10Checked`] directly if you do not want to allow signs. Use
/// [`FromRadix10`] or [`FromRadix10Signed`] if you want to opt out overflow / underflow checking.
///
/// # Example
///
/// ```
/// use atoi::atoi;
/// // Parsing to digits from a slice
/// assert_eq!(Some(42), atoi::<u32>(b"42"));
/// // Additional bytes after the number are ignored. If you want to know how many bytes were used
/// // to parse the number use `FromRadix10::from_radix_10`.
/// assert_eq!(Some(42), atoi::<u32>(b"42 is the answer to life, the universe and everything"));
/// // `None` is returned if the slice does not start with a digit
/// assert_eq!(None, atoi::<u32>(b"Sadly we do not know the question"));
/// // While signed integer types are supported...
/// assert_eq!(Some(42), atoi::<i32>(b"42"));
/// // Signs are allowed.
/// assert_eq!(Some(-42), atoi::<i32>(b"-42"));
/// // Leading zeros are allowed
/// assert_eq!(Some(42), atoi::<u32>(b"0042"));
/// // Overflows will return `None`
/// assert_eq!(None, atoi::<u8>(b"256"));
/// ```
///
/// # Return
///
/// Returns a a number if the slice started with a number, otherwise `None` is returned.
pub fn atoi<I>(text: &[u8]) -> Option<I>
where
    I: FromRadix10SignedChecked,
{
    match I::from_radix_10_signed_checked(text) {
        (_, 0) | (None, _) => None,
        (Some(n), _) => Some(n),
    }
}
