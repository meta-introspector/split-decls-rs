use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Creates an iterator from a fallible function generating values.
///
/// ```
/// # use fallible_iterator::{from_fn, FallibleIterator};
/// let mut count = 0;
/// let counter = from_fn(move || {
///     // Increment our count. This is why we started at zero.
///     count += 1;
///
///     // Check to see if we've finished counting or not.
///     if count < 6 {
///         Ok(Some(count))
///     } else if count < 7 {
///         Ok(None)
///     } else {
///         Err(())
///     }
/// });
/// assert_eq!(&counter.collect::<Vec<_>>().unwrap(), &[1, 2, 3, 4, 5]);
/// ```
#[inline]
pub fn from_fn<I, E, F>(fun: F) -> FromFn<F>
where
    F: FnMut() -> Result<Option<I>, E>,
{
    FromFn { fun }
}
