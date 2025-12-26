use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<I> FromRadix10SignedChecked for I
where
    I: Zero
        + One
        + AddAssign
        + MulAssign
        + SubAssign
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + MaxNumDigits,
{
    fn from_radix_10_signed_checked(text: &[u8]) -> (Option<Self>, usize) {
        let mut index;
        let mut number = I::zero();
        let (sign, offset) = text
            .first()
            .and_then(|&byte| Sign::try_from(byte))
            .map(|sign| (sign, 1))
            .unwrap_or((Sign::Plus, 0));
        index = offset;
        match sign {
            Sign::Plus => {
                let max_safe_digits = max(1, I::max_num_digits(nth(10))) - 1;
                let max_safe_index = min(text.len(), max_safe_digits + offset);
                while index != max_safe_index {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number += digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                let mut number = Some(number);
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(&nth(10)));
                        number = number.and_then(|n| n.checked_add(&digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
            Sign::Minus => {
                let max_safe_digits = max(1, I::max_num_digits_negative(nth(10))) - 1;
                let max_safe_index = min(text.len(), max_safe_digits + offset);
                while index != max_safe_index {
                    if let Some(digit) = ascii_to_digit::<I>(text[index]) {
                        number *= nth(10);
                        number -= digit;
                        index += 1;
                    } else {
                        break;
                    }
                }
                let mut number = Some(number);
                while index != text.len() {
                    if let Some(digit) = ascii_to_digit(text[index]) {
                        number = number.and_then(|n| n.checked_mul(&nth(10)));
                        number = number.and_then(|n| n.checked_sub(&digit));
                        index += 1;
                    } else {
                        break;
                    }
                }
                (number, index)
            }
        }
    }
}
