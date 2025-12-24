use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
/// [Chain][`chain`] zero or more iterators together into one sequence.
///
/// The comma-separated arguments must implement [`IntoIterator`].
/// The final argument may be followed by a trailing comma.
///
/// [`chain`]: Iterator::chain
///
/// # Examples
///
/// Empty invocations of `chain!` expand to an invocation of [`std::iter::empty`]:
/// ```
/// use itertools::chain;
/// use std::iter;
///
/// let _: iter::Empty<()> = chain!();
/// let _: iter::Empty<i8> = chain!();
/// ```
///
/// Invocations of `chain!` with one argument expand to [`arg.into_iter()`](IntoIterator):
/// ```
/// use itertools::chain;
/// use std::ops::Range;
/// let _: <Range<_> as IntoIterator>::IntoIter = chain!(2..6,); // trailing comma optional!
/// let _:     <&[_] as IntoIterator>::IntoIter = chain!(&[2, 3, 4]);
/// ```
///
/// Invocations of `chain!` with multiple arguments [`.into_iter()`](IntoIterator) each
/// argument, and then [`chain`] them together:
/// ```
/// use itertools::{assert_equal, chain};
/// use std::{iter::*, slice};
///
/// // e.g., this:
/// let with_macro: Chain<Chain<Once<_>, Take<Repeat<_>>>, slice::Iter<_>> =
///     chain![once(&0), repeat(&1).take(2), &[2, 3, 5],];
///
/// // ...is equivalent to this:
/// let with_method: Chain<Chain<Once<_>, Take<Repeat<_>>>, slice::Iter<_>> =
///     once(&0)
///         .chain(repeat(&1).take(2))
///         .chain(&[2, 3, 5]);
///
/// assert_equal(with_macro, with_method);
/// ```
macro_rules! chain {
    () => {
        $crate::__std_iter::empty()
    };
    ($first:expr $(, $rest:expr)* $(,)?) => {
        { let iter = $crate::__std_iter::IntoIterator::into_iter($first); $(let iter =
        $crate::__std_iter::Iterator::chain(iter,
        $crate::__std_iter::IntoIterator::into_iter($rest));)* iter }
    };
}
