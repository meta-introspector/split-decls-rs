use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Sign {
    /// Trys to convert an ascii character into a `Sign`
    ///
    /// # Example
    ///
    /// ```
    /// use atoi::Sign;
    /// assert_eq!(Some(Sign::Plus), Sign::try_from(b'+'));
    /// assert_eq!(Some(Sign::Minus), Sign::try_from(b'-'));
    /// assert_eq!(None, Sign::try_from(b'1'));
    /// ```
    pub fn try_from(byte: u8) -> Option<Sign> {
        match byte {
            b'+' => Some(Sign::Plus),
            b'-' => Some(Sign::Minus),
            _ => None,
        }
    }
    /// Returns either `+1` or `-1`
    pub fn signum<I>(self) -> I
    where
        I: Signed,
    {
        match self {
            Sign::Plus => I::one(),
            Sign::Minus => -I::one(),
        }
    }
}
