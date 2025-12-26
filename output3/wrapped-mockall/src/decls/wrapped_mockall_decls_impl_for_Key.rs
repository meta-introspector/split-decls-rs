use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
impl Key {
    pub fn new<T: 'static + ?Sized>() -> Self {
        Key(any::TypeId::of::<T>())
    }
}
