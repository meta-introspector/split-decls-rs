use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> IdxRange<T> {
    /// Creates a new index range
    /// inclusive of the start value and exclusive of the end value.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let a = arena.alloc("a");
    /// let b = arena.alloc("b");
    /// let c = arena.alloc("c");
    /// let d = arena.alloc("d");
    ///
    /// let range = la_arena::IdxRange::new(b..d);
    /// assert_eq!(&arena[range], &["b", "c"]);
    /// ```
    pub fn new(range: Range<Idx<T>>) -> Self {
        Self {
            range: range.start.into_raw().into()..range.end.into_raw().into(),
            _p: PhantomData,
        }
    }
    /// Creates a new index range
    /// inclusive of the start value and end value.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let foo = arena.alloc("foo");
    /// let bar = arena.alloc("bar");
    /// let baz = arena.alloc("baz");
    ///
    /// let range = la_arena::IdxRange::new_inclusive(foo..=baz);
    /// assert_eq!(&arena[range], &["foo", "bar", "baz"]);
    ///
    /// let range = la_arena::IdxRange::new_inclusive(foo..=foo);
    /// assert_eq!(&arena[range], &["foo"]);
    /// ```
    pub fn new_inclusive(range: RangeInclusive<Idx<T>>) -> Self {
        Self {
            range: u32::from(range.start().into_raw())..u32::from(range.end().into_raw()) + 1,
            _p: PhantomData,
        }
    }
    /// Returns whether the index range is empty.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let one = arena.alloc(1);
    /// let two = arena.alloc(2);
    ///
    /// assert!(la_arena::IdxRange::new(one..one).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
    /// Returns the start of the index range.
    pub fn start(&self) -> Idx<T> {
        Idx::from_raw(RawIdx::from(self.range.start))
    }
    /// Returns the end of the index range.
    pub fn end(&self) -> Idx<T> {
        Idx::from_raw(RawIdx::from(self.range.end))
    }
}
