use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Partition a sequence using predicate `pred` so that elements
/// that map to `true` are placed before elements which map to `false`.
///
/// The order within the partitions is arbitrary.
///
/// Return the index of the split point.
///
/// ```
/// use itertools::partition;
///
/// # // use repeated numbers to not promise any ordering
/// let mut data = [7, 1, 1, 7, 1, 1, 7];
/// let split_index = partition(&mut data, |elt| *elt >= 3);
///
/// assert_eq!(data, [7, 7, 7, 1, 1, 1, 1]);
/// assert_eq!(split_index, 3);
/// ```
pub fn partition<'a, A: 'a, I, F>(iter: I, mut pred: F) -> usize
where
    I: IntoIterator<Item = &'a mut A>,
    I::IntoIter: DoubleEndedIterator,
    F: FnMut(&A) -> bool,
{
    let mut split_index = 0;
    let mut iter = iter.into_iter();
    while let Some(front) = iter.next() {
        if !pred(front) {
            match iter.rfind(|back| pred(back)) {
                Some(back) => std::mem::swap(front, back),
                None => break,
            }
        }
        split_index += 1;
    }
    split_index
}
