use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T: 'a, N: ArrayLength> IntoIterator for &'a mut GenericArray<T, N> {
    type IntoIter = slice::IterMut<'a, T>;
    type Item = &'a mut T;
    #[inline]
    fn into_iter(self: &'a mut GenericArray<T, N>) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}
