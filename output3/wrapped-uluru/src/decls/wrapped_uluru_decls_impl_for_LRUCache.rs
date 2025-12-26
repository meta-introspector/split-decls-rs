use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> LRUCache<T, N> {
    /// Create an empty cache.
    pub const fn new() -> Self {
        assert!(N < u16::MAX as usize, "Capacity overflow");
        LRUCache {
            entries: ArrayVec::new_const(),
            head: 0,
            tail: 0,
        }
    }
    /// Insert a given key in the cache.
    ///
    /// This item becomes the front (most-recently-used) item in the cache.  If the cache is full,
    /// the back (least-recently-used) item will be removed and returned.
    pub fn insert(&mut self, val: T) -> Option<T> {
        let new_entry = Entry {
            val,
            prev: 0,
            next: 0,
        };
        if self.entries.is_full() {
            let i = self.pop_back();
            let old_entry = replace(self.entry(i), new_entry);
            self.push_front(i);
            Some(old_entry.val)
        } else {
            let i = self.entries.len() as u16;
            self.entries.push(new_entry);
            self.push_front(i);
            None
        }
    }
    /// Returns the first item in the cache that matches the given predicate.
    /// Touches the result (makes it most-recently-used) on a hit.
    pub fn find<F>(&mut self, pred: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> bool,
    {
        if self.touch(pred) {
            self.front_mut()
        } else {
            None
        }
    }
    /// Performs a lookup on the cache with the given test routine. Touches
    /// the result on a hit.
    pub fn lookup<F, R>(&mut self, mut pred: F) -> Option<R>
    where
        F: FnMut(&mut T) -> Option<R>,
    {
        let mut iter = self.iter_mut();
        while let Some((i, val)) = iter.next() {
            if let Some(r) = pred(val) {
                self.touch_index(i);
                return Some(r);
            }
        }
        None
    }
    /// Returns the number of elements in the cache.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if the cache is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Evict all elements from the cache.
    #[inline]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Returns the front entry in the list (most recently used).
    pub fn front(&self) -> Option<&T> {
        self.entries.get(self.head as usize).map(|e| &e.val)
    }
    /// Returns a mutable reference to the front entry in the list (most recently used).
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.entries.get_mut(self.head as usize).map(|e| &mut e.val)
    }
    /// Returns the n-th entry in the list (most recently used).
    pub fn get(&self, index: usize) -> Option<&T> {
        self.iter().nth(index)
    }
    /// Touches the first item in the cache that matches the given predicate (marks it as
    /// most-recently-used).
    /// Returns `true` on a hit, `false` if no matches.
    pub fn touch<F>(&mut self, mut pred: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        let mut iter = self.iter_mut();
        while let Some((i, val)) = iter.next() {
            if pred(val) {
                self.touch_index(i);
                return true;
            }
        }
        false
    }
    /// Iterate over the contents of this cache in order from most-recently-used to
    /// least-recently-used.
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            pos: self.head,
            cache: self,
        }
    }
    /// Iterate mutably over the contents of this cache in order from most-recently-used to
    /// least-recently-used.
    fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            pos: self.head,
            cache: self,
        }
    }
    /// Touch a given entry, putting it first in the list.
    #[inline]
    fn touch_index(&mut self, i: u16) {
        if i != self.head {
            self.remove(i);
            self.push_front(i);
        }
    }
    #[inline(always)]
    fn entry(&mut self, i: u16) -> &mut Entry<T> {
        &mut self.entries[i as usize]
    }
    /// Remove an entry from the linked list.
    ///
    /// Note: This only unlinks the entry from the list; it does not remove it from the array.
    fn remove(&mut self, i: u16) {
        let prev = self.entry(i).prev;
        let next = self.entry(i).next;
        if i == self.head {
            self.head = next;
        } else {
            self.entry(prev).next = next;
        }
        if i == self.tail {
            self.tail = prev;
        } else {
            self.entry(next).prev = prev;
        }
    }
    /// Insert a new entry at the head of the list.
    fn push_front(&mut self, i: u16) {
        if self.entries.len() == 1 {
            self.tail = i;
        } else {
            self.entry(i).next = self.head;
            self.entry(self.head).prev = i;
        }
        self.head = i;
    }
    /// Remove the last entry from the linked list. Returns the index of the removed entry.
    ///
    /// Note: This only unlinks the entry from the list; it does not remove it from the array.
    fn pop_back(&mut self) -> u16 {
        let new_tail = self.entry(self.tail).prev;
        replace(&mut self.tail, new_tail)
    }
}
