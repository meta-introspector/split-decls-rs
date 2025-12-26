use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
)]
pub enum IntegerType {
    /// Pointer-sized integer type, i.e. `isize` and `usize`. The field shows signedness, e.g.
    /// `Pointer(true)` means `isize`.
    Pointer(bool),
    /// Fixed-sized integer type, e.g. `i8`, `u32`, `i128`. The bool field shows signedness, e.g.
    /// `Fixed(I8, false)` means `u8`.
    Fixed(Integer, bool),
}
