use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A draining iterator for `ThinVec<T>`.
///
/// This `struct` is created by [`ThinVec::drain`].
/// See its documentation for more.
///
/// # Example
///
/// ```
/// use thin_vec::thin_vec;
///
/// let mut v = thin_vec![0, 1, 2];
/// let iter: thin_vec::Drain<_> = v.drain(..);
/// ```
pub struct Drain<'a, T> {
    /// An iterator over the elements we're removing.
    ///
    /// As we go we'll be `read`ing out of the shared refs yielded by this.
    /// It's ok to use Iter here because it promises to only take refs to the parts
    /// we haven't yielded yet.
    iter: Iter<'a, T>,
    /// The actual ThinVec, which we need to hold onto to undo the leak amplification
    /// and backshift the tail into place. This should only be accessed when we're
    /// completely done with the Iter in the `drop` impl of this type (or miri will get mad).
    ///
    /// Since we set the `len` of this to be before `Iter`, we can use that `len`
    /// to retrieve the index of the start of the drain range later.
    vec: NonNull<ThinVec<T>>,
    /// The one-past-the-end index of the drain range, or equivalently the start of the tail.
    end: usize,
    /// The length of the tail.
    tail: usize,
}
