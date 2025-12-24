use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct BytesToHexChars<'a> {
    inner: core::slice::Iter<'a, u8>,
    table: &'static [u8; 16],
    next: Option<char>,
}
