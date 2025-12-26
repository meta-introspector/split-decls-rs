use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Yields all [`Hunk`]s in a file in monotonically increasing order.
/// Monotonically increasing means here that the following holds for any two
/// consecutive [`Hunk`]s `x` and `y`:
///
/// ``` no_compile
/// assert!(x.before.end < y.before.start);
/// assert!(x.after.end < y.after.start);
/// ```
///
pub struct HunkIter<'diff> {
    removed: slice::Iter<'diff, bool>,
    added: slice::Iter<'diff, bool>,
    pos_before: u32,
    pos_after: u32,
}
