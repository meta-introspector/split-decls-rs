use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which flattens an iterator of iterators, yielding those iterators' elements.
pub struct Flatten<I>
where
    I: FallibleIterator,
    I::Item: IntoFallibleIterator,
{
    it: I,
    cur: Option<<I::Item as IntoFallibleIterator>::IntoFallibleIter>,
}
