use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Arena<T> {
    /// Creates a new empty arena.
    ///
    /// ```
    /// let arena: la_arena::Arena<i32> = la_arena::Arena::new();
    /// assert!(arena.is_empty());
    /// ```
    pub const fn new() -> Arena<T> {
        Arena { data: Vec::new() }
    }
    /// Create a new empty arena with specific capacity.
    ///
    /// ```
    /// let arena: la_arena::Arena<i32> = la_arena::Arena::with_capacity(42);
    /// assert!(arena.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Arena<T> {
        Arena {
            data: Vec::with_capacity(capacity),
        }
    }
    /// Empties the arena, removing all contained values.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    ///
    /// arena.alloc(1);
    /// arena.alloc(2);
    /// arena.alloc(3);
    /// assert_eq!(arena.len(), 3);
    ///
    /// arena.clear();
    /// assert!(arena.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
    }
    /// Returns the length of the arena.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// assert_eq!(arena.len(), 0);
    ///
    /// arena.alloc("foo");
    /// assert_eq!(arena.len(), 1);
    ///
    /// arena.alloc("bar");
    /// assert_eq!(arena.len(), 2);
    ///
    /// arena.alloc("baz");
    /// assert_eq!(arena.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns whether the arena contains no elements.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// assert!(arena.is_empty());
    ///
    /// arena.alloc(0.5);
    /// assert!(!arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Allocates a new value on the arena, returning the value’s index.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let idx = arena.alloc(50);
    ///
    /// assert_eq!(arena[idx], 50);
    /// ```
    pub fn alloc(&mut self, value: T) -> Idx<T> {
        let idx = self.next_idx();
        self.data.push(value);
        idx
    }
    /// Densely allocates multiple values, returning the values’ index range.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let range = arena.alloc_many(0..4);
    ///
    /// assert_eq!(arena[range], [0, 1, 2, 3]);
    /// ```
    pub fn alloc_many<II: IntoIterator<Item = T>>(&mut self, iter: II) -> IdxRange<T> {
        let start = self.next_idx();
        self.extend(iter);
        let end = self.next_idx();
        IdxRange::new(start..end)
    }
    /// Returns an iterator over the arena’s elements.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let idx1 = arena.alloc(20);
    /// let idx2 = arena.alloc(40);
    /// let idx3 = arena.alloc(60);
    ///
    /// let mut iterator = arena.iter();
    /// assert_eq!(iterator.next(), Some((idx1, &20)));
    /// assert_eq!(iterator.next(), Some((idx2, &40)));
    /// assert_eq!(iterator.next(), Some((idx3, &60)));
    /// ```
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (Idx<T>, &T)> + DoubleEndedIterator + Clone {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, value)| (Idx::from_raw(RawIdx(idx as u32)), value))
    }
    /// Returns an iterator over the arena’s mutable elements.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let idx1 = arena.alloc(20);
    ///
    /// assert_eq!(arena[idx1], 20);
    ///
    /// let mut iterator = arena.iter_mut();
    /// *iterator.next().unwrap().1 = 10;
    /// drop(iterator);
    ///
    /// assert_eq!(arena[idx1], 10);
    /// ```
    pub fn iter_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = (Idx<T>, &mut T)> + DoubleEndedIterator {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(idx, value)| (Idx::from_raw(RawIdx(idx as u32)), value))
    }
    /// Returns an iterator over the arena’s values.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let idx1 = arena.alloc(20);
    /// let idx2 = arena.alloc(40);
    /// let idx3 = arena.alloc(60);
    ///
    /// let mut iterator = arena.values();
    /// assert_eq!(iterator.next(), Some(&20));
    /// assert_eq!(iterator.next(), Some(&40));
    /// assert_eq!(iterator.next(), Some(&60));
    /// ```
    pub fn values(&self) -> impl ExactSizeIterator<Item = &T> + DoubleEndedIterator {
        self.data.iter()
    }
    /// Returns an iterator over the arena’s mutable values.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let idx1 = arena.alloc(20);
    ///
    /// assert_eq!(arena[idx1], 20);
    ///
    /// let mut iterator = arena.values_mut();
    /// *iterator.next().unwrap() = 10;
    /// drop(iterator);
    ///
    /// assert_eq!(arena[idx1], 10);
    /// ```
    pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> + DoubleEndedIterator {
        self.data.iter_mut()
    }
    /// Reallocates the arena to make it take up as little space as possible.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    /// Returns the index of the next value allocated on the arena.
    ///
    /// This method should remain private to make creating invalid `Idx`s harder.
    fn next_idx(&self) -> Idx<T> {
        Idx::from_raw(RawIdx(self.data.len() as u32))
    }
}
