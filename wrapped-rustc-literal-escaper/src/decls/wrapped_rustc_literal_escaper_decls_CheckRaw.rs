use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait for checking raw string literals for validity
trait CheckRaw {
    /// Unit type of the implementing string type (`char` for string, `u8` for byte string)
    type RawUnit;
    /// Converts chars to the unit type of the literal type
    fn char2raw_unit(c: char) -> Result<Self::RawUnit, EscapeError>;
    /// Takes the contents of a raw literal (without quotes)
    /// and produces a sequence of `Result<Self::RawUnit, EscapeError>`
    /// which are returned via `callback`.
    ///
    /// NOTE: Does no escaping, but produces errors for bare carriage return ('\r').
    fn check_raw(
        src: &str,
        mut callback: impl FnMut(Range<usize>, Result<Self::RawUnit, EscapeError>),
    ) {
        let mut chars = src.chars();
        while let Some(c) = chars.next() {
            let start = src.len() - chars.as_str().len() - c.len_utf8();
            let res = match c {
                '\r' => Err(EscapeError::BareCarriageReturnInRawString),
                _ => Self::char2raw_unit(c),
            };
            let end = src.len() - chars.as_str().len();
            callback(start..end, res);
        }
    }
}
