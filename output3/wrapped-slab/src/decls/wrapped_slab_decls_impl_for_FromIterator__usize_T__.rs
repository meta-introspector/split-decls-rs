use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Create a slab from an iterator of key-value pairs.
///
/// If the iterator produces duplicate keys, the previous value is replaced with the later one.
/// The keys does not need to be sorted beforehand, and this function always
/// takes O(n) time.
/// Note that the returned slab will use space proportional to the largest key,
/// so don't use `Slab` with untrusted keys.
///
/// # Examples
///
/// ```
/// # use slab::*;
///
/// let vec = vec![(2,'a'), (6,'b'), (7,'c')];
/// let slab = vec.into_iter().collect::<Slab<char>>();
/// assert_eq!(slab.len(), 3);
/// assert!(slab.capacity() >= 8);
/// assert_eq!(slab[2], 'a');
/// ```
///
/// With duplicate and unsorted keys:
///
/// ```
/// # use slab::*;
///
/// let vec = vec![(20,'a'), (10,'b'), (11,'c'), (10,'d')];
/// let slab = vec.into_iter().collect::<Slab<char>>();
/// assert_eq!(slab.len(), 3);
/// assert_eq!(slab[10], 'd');
/// ```
impl<T> FromIterator<(usize, T)> for Slab<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (usize, T)>,
    {
        let iterator = iterable.into_iter();
        let mut builder = builder::Builder::with_capacity(iterator.size_hint().0);
        for (key, value) in iterator {
            builder.pair(key, value);
        }
        builder.build()
    }
}
