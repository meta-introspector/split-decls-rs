use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn common_suffix(text1: Range, text2: Range) -> usize {
    for (i, (b1, b2)) in text1.chars().rev().zip(text2.chars().rev()).enumerate() {
        if b1 != b2 {
            return i;
        }
    }
    cmp::min(text1.len, text2.len)
}
