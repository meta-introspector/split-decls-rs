use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawStrError {
    /// Non `#` characters exist between `r` and `"`, e.g. `r##~"abcde"##`
    InvalidStarter { bad_char: char },
    /// The string was not terminated, e.g. `r###"abcde"##`.
    /// `possible_terminator_offset` is the number of characters after `r` or
    /// `br` where they may have intended to terminate it.
    NoTerminator {
        expected: u32,
        found: u32,
        possible_terminator_offset: Option<u32>,
    },
    /// More than 255 `#`s exist.
    TooManyDelimiters { found: u32 },
}
