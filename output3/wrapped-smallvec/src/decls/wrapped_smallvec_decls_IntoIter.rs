use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that consumes a `SmallVec` and yields its items by value.
///
/// Returned from [`SmallVec::into_iter`][1].
///
/// [1]: struct.SmallVec.html#method.into_iter
pub struct IntoIter<T, const N: usize> {
    raw: RawSmallVec<T, N>,
    begin: usize,
    end: TaggedLen,
    _marker: PhantomData<T>,
}
