use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> TypedArena<T> {
    /// Allocates an object in the `TypedArena`, returning a reference to it.
    #[inline]
    pub fn alloc(&self, object: T) -> &mut T {
        if self.ptr == self.end {
            self.grow(1)
        }
        unsafe {
            if size_of::<T>() == 0 {
                self.ptr.set(self.ptr.get().wrapping_byte_add(1));
                let ptr = ptr::NonNull::<T>::dangling().as_ptr();
                ptr::write(ptr, object);
                &mut *ptr
            } else {
                let ptr = self.ptr.get();
                self.ptr.set(self.ptr.get().add(1));
                ptr::write(ptr, object);
                &mut *ptr
            }
        }
    }
    #[inline]
    fn can_allocate(&self, additional: usize) -> bool {
        let available_bytes = self.end.get().addr() - self.ptr.get().addr();
        let additional_bytes = additional.checked_mul(size_of::<T>()).unwrap();
        available_bytes >= additional_bytes
    }
    #[inline]
    fn alloc_raw_slice(&self, len: usize) -> *mut T {
        assert!(size_of::<T>() != 0);
        assert!(len != 0);
        if !self.can_allocate(len) {
            self.grow(len);
            debug_assert!(self.can_allocate(len));
        }
        let start_ptr = self.ptr.get();
        unsafe { self.ptr.set(start_ptr.add(len)) };
        start_ptr
    }
    /// Allocates the elements of this iterator into a contiguous slice in the `TypedArena`.
    ///
    /// Note: for reasons of reentrancy and panic safety we collect into a `SmallVec<_, 8>` before
    /// storing the elements in the arena.
    #[inline]
    pub fn alloc_from_iter<I: IntoIterator<Item = T>>(&self, iter: I) -> &mut [T] {
        self.try_alloc_from_iter(iter.into_iter().map(Ok::<T, !>))
            .into_ok()
    }
    /// Allocates the elements of this iterator into a contiguous slice in the `TypedArena`.
    ///
    /// Note: for reasons of reentrancy and panic safety we collect into a `SmallVec<_, 8>` before
    /// storing the elements in the arena.
    #[inline]
    pub fn try_alloc_from_iter<E>(
        &self,
        iter: impl IntoIterator<Item = Result<T, E>>,
    ) -> Result<&mut [T], E> {
        let vec: Result<SmallVec<T, 8>, E> = iter.into_iter().collect();
        let mut vec = vec?;
        if vec.is_empty() {
            return Ok(&mut []);
        }
        let len = vec.len();
        let start_ptr = self.alloc_raw_slice(len);
        Ok(unsafe {
            vec.as_ptr().copy_to_nonoverlapping(start_ptr, len);
            vec.set_len(0);
            slice::from_raw_parts_mut(start_ptr, len)
        })
    }
    /// Grows the arena.
    #[inline(never)]
    #[cold]
    fn grow(&self, additional: usize) {
        unsafe {
            let elem_size = cmp::max(1, size_of::<T>());
            let mut chunks = self.chunks.borrow_mut();
            let mut new_cap;
            if let Some(last_chunk) = chunks.last_mut() {
                if mem::needs_drop::<T>() {
                    let used_bytes = self.ptr.get().addr() - last_chunk.start().addr();
                    last_chunk.entries = used_bytes / size_of::<T>();
                }
                new_cap = last_chunk.storage.len().min(HUGE_PAGE / elem_size / 2);
                new_cap *= 2;
            } else {
                new_cap = PAGE / elem_size;
            }
            new_cap = cmp::max(additional, new_cap);
            let mut chunk = ArenaChunk::<T>::new(new_cap);
            self.ptr.set(chunk.start());
            self.end.set(chunk.end());
            chunks.push(chunk);
        }
    }
    fn clear_last_chunk(&self, last_chunk: &mut ArenaChunk<T>) {
        let start = last_chunk.start().addr();
        let end = self.ptr.get().addr();
        let diff = if size_of::<T>() == 0 {
            end - start
        } else {
            (end - start) / size_of::<T>()
        };
        unsafe {
            last_chunk.destroy(diff);
        }
        self.ptr.set(last_chunk.start());
    }
}
