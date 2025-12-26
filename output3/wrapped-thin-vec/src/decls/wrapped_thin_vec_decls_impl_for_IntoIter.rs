use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> IntoIter<T> {
    /// Returns the remaining items of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::thin_vec;
    ///
    /// let vec = thin_vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &['b', 'c']);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.vec.data_raw().add(self.start), self.len()) }
    }
    /// Returns the remaining items of this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::thin_vec;
    ///
    /// let vec = thin_vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// into_iter.as_mut_slice()[2] = 'z';
    /// assert_eq!(into_iter.next().unwrap(), 'a');
    /// assert_eq!(into_iter.next().unwrap(), 'b');
    /// assert_eq!(into_iter.next().unwrap(), 'z');
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }
    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        unsafe { ptr::slice_from_raw_parts_mut(self.vec.data_raw().add(self.start), self.len()) }
    }
}
