use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn iter_eq_by<I, I2, F>(this: I2, other: I, mut eq: F) -> bool
where
    I: IntoIterator,
    I2: IntoIterator,
    F: FnMut(I2::Item, I::Item) -> bool,
{
    let mut other = other.into_iter();
    let mut this = this.into_iter();
    loop {
        let x = match this.next() {
            None => return other.next().is_none(),
            Some(val) => val,
        };
        let y = match other.next() {
            None => return false,
            Some(val) => val,
        };
        if !eq(x, y) {
            return false;
        }
    }
}
