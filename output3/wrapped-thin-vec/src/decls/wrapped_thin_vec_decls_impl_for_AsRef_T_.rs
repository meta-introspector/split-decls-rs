use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T> AsRef<[T]> for Drain<'a, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
