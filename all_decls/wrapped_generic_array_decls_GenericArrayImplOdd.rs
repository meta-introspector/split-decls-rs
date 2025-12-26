use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Internal type used to generate a struct of appropriate size
#[allow(dead_code)]
#[repr(C)]
#[doc(hidden)]
pub struct GenericArrayImplOdd<T, U> {
    parents: [U; 2],
    data: T,
}
