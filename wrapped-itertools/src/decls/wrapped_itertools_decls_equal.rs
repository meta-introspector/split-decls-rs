use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return `true` if both iterables produce equal sequences
/// (elements pairwise equal and sequences of the same length),
/// `false` otherwise.
///
/// [`IntoIterator`] enabled version of [`Iterator::eq`].
///
/// ```
/// assert!(itertools::equal(vec![1, 2, 3], 1..4));
/// assert!(!itertools::equal(&[0, 0], &[0, 0, 0]));
/// ```
pub fn equal<I, J>(a: I, b: J) -> bool
where
    I: IntoIterator,
    J: IntoIterator,
    I::Item: PartialEq<J::Item>,
{
    a.into_iter().eq(b)
}
