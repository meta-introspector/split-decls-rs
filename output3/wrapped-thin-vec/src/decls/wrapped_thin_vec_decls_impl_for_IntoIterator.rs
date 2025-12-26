use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T> IntoIterator for &'a mut ThinVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.iter_mut()
    }
}
