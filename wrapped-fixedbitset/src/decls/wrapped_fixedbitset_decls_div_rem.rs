use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[inline]
fn div_rem(x: usize, denominator: usize) -> (usize, usize) {
    (x / denominator, x % denominator)
}
