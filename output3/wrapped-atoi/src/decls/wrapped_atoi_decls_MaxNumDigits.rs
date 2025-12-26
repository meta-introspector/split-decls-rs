use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A bounded integer, whose representation can overflow and therefore can only store a maximum
/// number of digits
pub trait MaxNumDigits {
    /// Given a representation with a radix character I, what is the maximum number of digits we can
    /// parse without the integer overflowing for sure?
    fn max_num_digits(radix: Self) -> usize;
    /// Returns the maximum number of digits a negative representation of `I` can have depending on
    /// `radix`.
    fn max_num_digits_negative(radix: Self) -> usize;
}
