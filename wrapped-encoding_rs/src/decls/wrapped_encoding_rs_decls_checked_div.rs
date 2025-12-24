use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[inline(always)]
fn checked_div(opt: Option<usize>, num: usize) -> Option<usize> {
    if let Some(n) = opt { n.checked_div(num) } else { None }
}
