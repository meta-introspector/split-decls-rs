use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<I> FromRadix16 for I
where
    I: Zero + One + AddAssign + MulAssign,
{
    fn from_radix_16(text: &[u8]) -> (Self, usize) {
        let mut index = 0;
        let mut number = I::zero();
        while index != text.len() {
            if let Some(digit) = ascii_to_hexdigit(text[index]) {
                number *= nth(16);
                number += digit;
                index += 1;
            } else {
                break;
            }
        }
        (number, index)
    }
}
