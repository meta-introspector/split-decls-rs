use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[inline(never)]
#[cold]
pub(crate) fn from_iter_length_fail(length: usize) -> ! {
    panic!("GenericArray::from_iter expected {length} items");
}
