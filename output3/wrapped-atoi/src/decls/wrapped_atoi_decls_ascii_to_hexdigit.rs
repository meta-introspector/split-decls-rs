use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts an ascii character to digit
fn ascii_to_hexdigit<I>(character: u8) -> Option<I>
where
    I: Zero + One,
{
    match character {
        b'0' => Some(nth(0)),
        b'1' => Some(nth(1)),
        b'2' => Some(nth(2)),
        b'3' => Some(nth(3)),
        b'4' => Some(nth(4)),
        b'5' => Some(nth(5)),
        b'6' => Some(nth(6)),
        b'7' => Some(nth(7)),
        b'8' => Some(nth(8)),
        b'9' => Some(nth(9)),
        b'a' | b'A' => Some(nth(10)),
        b'b' | b'B' => Some(nth(11)),
        b'c' | b'C' => Some(nth(12)),
        b'd' | b'D' => Some(nth(13)),
        b'e' | b'E' => Some(nth(14)),
        b'f' | b'F' => Some(nth(15)),
        _ => None,
    }
}
