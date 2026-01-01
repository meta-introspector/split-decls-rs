# AST Trace: ../rust/library/core/src/slice/iter.rs

Generated 205 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
//! Definitions of a bunch of iterators for `[T]`.

#[macro_use] // import iterator! and forward_iterator!
mod macros;

use super::{from_raw_parts, from_raw_parts_mut};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::hint::assert_unchecked;
use crate::iter::{
    FusedIterator, TrustedLen, TrustedRandomAccess, TrustedRandomAccessNoCoerce, UncheckedIterator,
};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::marker::PhantomData;
use crate::mem::{self, SizedTypeProperties};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::num::NonZero;
use crate::ptr::{NonNull, without_provenance, without_provenance_mut};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{cmp, fmt};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<T> !Iterator for [T] {}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=into_iter | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a [T] {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=into_iter | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a mut [T] {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=18

```rust
/// Immutable slice iterator
///
/// This struct is created by the [`iter`] method on [slices].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // First, we need a slice to call the `iter` method on:
/// let slice = &[1, 2, 3];
///
/// // Then we call `iter` on the slice to get the `Iter` iterator,
/// // and iterate over it:
/// for element in slice.iter() {
///     println!("{element}");
/// }
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=5

```rust
///
/// // This for loop actually already works without calling `iter`:
/// for element in slice {
///     println!("{element}");
/// }
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=Iter | COMPLEXITY=7 | LINES=19

```rust
/// ```
///
/// [`iter`]: slice::iter
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[rustc_diagnostic_item = "SliceIter"]
pub struct Iter<'a, T: 'a> {
    /// The pointer to the next element to return, or the past-the-end location
    /// if the iterator is empty.
    ///
    /// This address will be used for all ZST elements, never changed.
    ptr: NonNull<T>,
    /// For non-ZSTs, the non-null pointer to the past-the-end element.
    ///
    /// For ZSTs, this is `ptr::without_provenance_mut(len)`.
    end_or_len: *const T,
    _marker: PhantomData<&'a T>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.as_slice()).finish()
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for Iter<'_, T> {}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=2

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Send for Iter<'_, T> {}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=as_slice | COMPLEXITY=20 | LINES=48

```rust
impl<'a, T> Iter<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T]) -> Self {
        let len = slice.len();
        let ptr: NonNull<T> = NonNull::from_ref(slice).cast();
        // SAFETY: Similar to `IterMut::new`.
        unsafe {
            let end_or_len =
                if T::IS_ZST { without_provenance(len) } else { ptr.as_ptr().add(len) };

            Self { ptr, end_or_len, _marker: PhantomData }
        }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // First, we need a slice to call the `iter` method on:
    /// let slice = &[1, 2, 3];
    ///
    /// // Then we call `iter` on the slice to get the `Iter` iterator:
    /// let mut iter = slice.iter();
    /// // Here `as_slice` still returns the whole slice, so this prints "[1, 2, 3]":
    /// println!("{:?}", iter.as_slice());
    ///
    /// // Now, we call the `next` method to remove the first element from the iterator:
    /// iter.next();
    /// // Here the iterator does not contain the first element of the slice any more,
    /// // so `as_slice` only returns the last two elements of the slice,
    /// // and so this prints "[2, 3]":
    /// println!("{:?}", iter.as_slice());
    ///
    /// // The underlying slice has not been modified and still contains three elements,
    /// // so this prints "[1, 2, 3]":
    /// println!("{:?}", slice);
    /// ```
    #[must_use]
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    #[inline]
    pub fn as_slice(&self) -> &'a [T] {
        self.make_slice()
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=is_sorted_by | COMPLEXITY=5 | LINES=10

```rust
iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, as_ref, {
    fn is_sorted_by<F>(self, mut compare: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        self.as_slice().is_sorted_by(|a, b| compare(&a, &b))
    }
}}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { ptr: self.ptr, end_or_len: self.end_or_len, _marker: self._marker }
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "slice_iter_as_ref", since = "1.13.0")]
impl<T> AsRef<[T]> for Iter<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=18

```rust
/// Mutable slice iterator.
///
/// This struct is created by the [`iter_mut`] method on [slices].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // First, we need a slice to call the `iter_mut` method on:
/// let slice = &mut [1, 2, 3];
///
/// // Then we call `iter_mut` on the slice to get the `IterMut` iterator,
/// // iterate over it and increment each element value:
/// for element in slice.iter_mut() {
///     *element += 1;
/// }
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=IterMut | COMPLEXITY=8 | LINES=21

```rust
///
/// // We now have "[2, 3, 4]":
/// println!("{slice:?}");
/// ```
///
/// [`iter_mut`]: slice::iter_mut
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMut<'a, T: 'a> {
    /// The pointer to the next element to return, or the past-the-end location
    /// if the iterator is empty.
    ///
    /// This address will be used for all ZST elements, never changed.
    ptr: NonNull<T>,
    /// For non-ZSTs, the non-null pointer to the past-the-end element.
    ///
    /// For ZSTs, this is `ptr::without_provenance_mut(len)`.
    end_or_len: *mut T,
    _marker: PhantomData<&'a mut T>,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut").field(&self.make_slice()).finish()
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=2

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Send> Send for IterMut<'_, T> {}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=into_slice | COMPLEXITY=56 | LINES=137

```rust
impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T]) -> Self {
        let len = slice.len();
        let ptr: NonNull<T> = NonNull::from_mut(slice).cast();
        // SAFETY: There are several things here:
        //
        // `ptr` has been obtained by `slice.as_ptr()` where `slice` is a valid
        // reference thus it is non-NUL and safe to use and pass to
        // `NonNull::new_unchecked` .
        //
        // Adding `slice.len()` to the starting pointer gives a pointer
        // at the end of `slice`. `end` will never be dereferenced, only checked
        // for direct pointer equality with `ptr` to check if the iterator is
        // done.
        //
        // In the case of a ZST, the end pointer is just the length.  It's never
        // used as a pointer at all, and thus it's fine to have no provenance.
        //
        // See the `next_unchecked!` and `is_empty!` macros as well as the
        // `post_inc_start` method for more information.
        unsafe {
            let end_or_len =
                if T::IS_ZST { without_provenance_mut(len) } else { ptr.as_ptr().add(len) };

            Self { ptr, end_or_len, _marker: PhantomData }
        }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// To avoid creating `&mut` references that alias, this is forced
    /// to consume the iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // First, we need a slice to call the `iter_mut` method on:
    /// let mut slice = &mut [1, 2, 3];
    ///
    /// // Then we call `iter_mut` on the slice to get the `IterMut` struct:
    /// let mut iter = slice.iter_mut();
    /// // Now, we call the `next` method to remove the first element of the iterator,
    /// // unwrap and dereference what we get from `next` and increase its value by 1:
    /// *iter.next().unwrap() += 1;
    /// // Here the iterator does not contain the first element of the slice any more,
    /// // so `into_slice` only returns the last two elements of the slice,
    /// // and so this prints "[2, 3]":
    /// println!("{:?}", iter.into_slice());
    /// // The underlying slice still contains three elements, but its first element
    /// // was increased by 1, so this prints "[2, 2, 3]":
    /// println!("{:?}", slice);
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    pub fn into_slice(self) -> &'a mut [T] {
        // SAFETY: the iterator was created from a mutable slice with pointer
        // `self.ptr` and length `len!(self)`. This guarantees that all the prerequisites
        // for `from_raw_parts_mut` are fulfilled.
        unsafe { from_raw_parts_mut(self.ptr.as_ptr(), len!(self)) }
    }

    /// Views the underlying data as a subslice of the original data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // First, we need a slice to call the `iter_mut` method on:
    /// let slice = &mut [1, 2, 3];
    ///
    /// // Then we call `iter_mut` on the slice to get the `IterMut` iterator:
    /// let mut iter = slice.iter_mut();
    /// // Here `as_slice` still returns the whole slice, so this prints "[1, 2, 3]":
    /// println!("{:?}", iter.as_slice());
    ///
    /// // Now, we call the `next` method to remove the first element from the iterator
    /// // and increment its value:
    /// *iter.next().unwrap() += 1;
    /// // Here the iterator does not contain the first element of the slice any more,
    /// // so `as_slice` only returns the last two elements of the slice,
    /// // and so this prints "[2, 3]":
    /// println!("{:?}", iter.as_slice());
    ///
    /// // The underlying slice still contains three elements, but its first element
    /// // was increased by 1, so this prints "[2, 2, 3]":
    /// println!("{:?}", slice);
    /// ```
    #[must_use]
    #[stable(feature = "slice_iter_mut_as_slice", since = "1.53.0")]
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.make_slice()
    }

    /// Views the underlying data as a mutable subslice of the original data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(slice_iter_mut_as_mut_slice)]
    ///
    /// let mut slice: &mut [usize] = &mut [1, 2, 3];
    ///
    /// // First, we get the iterator:
    /// let mut iter = slice.iter_mut();
    /// // Then, we get a mutable slice from it:
    /// let mut_slice = iter.as_mut_slice();
    /// // So if we check what the `as_mut_slice` method returned, we have "[1, 2, 3]":
    /// assert_eq!(mut_slice, &mut [1, 2, 3]);
    ///
    /// // We can use it to mutate the slice:
    /// mut_slice[0] = 4;
    /// mut_slice[2] = 5;
    ///
    /// // Next, we can move to the second element of the slice, checking that
    /// // it yields the value we just wrote:
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// // Now `as_mut_slice` returns "[2, 5]":
    /// assert_eq!(iter.as_mut_slice(), &mut [2, 5]);
    /// ```
    #[must_use]
    // FIXME: Uncomment the `AsMut<[T]>` impl when this gets stabilized.
    #[unstable(feature = "slice_iter_mut_as_mut_slice", issue = "93079")]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: the iterator was created from a mutable slice with pointer
        // `self.ptr` and length `len!(self)`. This guarantees that all the prerequisites
        // for `from_raw_parts_mut` are fulfilled.
        unsafe { from_raw_parts_mut(self.ptr.as_ptr(), len!(self)) }
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "slice_iter_mut_as_slice", since = "1.53.0")]
impl<T> AsRef<[T]> for IterMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
// #[stable(feature = "slice_iter_mut_as_mut_slice", since = "FIXME")]
// impl<T> AsMut<[T]> for IterMut<'_, T> {
//     fn as_mut(&mut self) -> &mut [T] {
//         self.as_mut_slice()
//     }
// }
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, as_mut, {}}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=2 | LINES=9

```rust
/// An internal abstraction over the splitting iterators, so that
/// splitn, splitn_mut etc can be implemented once.
#[doc(hidden)]
pub(super) trait SplitIter: DoubleEndedIterator {
    /// Marks the underlying iterator as complete, extracting the remaining
    /// portion of the slice.
    fn finish(&mut self) -> Option<Self::Item>;
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=STRUCT | NAME=Split | COMPLEXITY=10 | LINES=30

```rust
/// An iterator over subslices separated by elements that match a predicate
/// function.
///
/// This struct is created by the [`split`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = [10, 40, 33, 20];
/// let mut iter = slice.split(|num| num % 3 == 0);
/// assert_eq!(iter.next(), Some(&[10, 40][..]));
/// assert_eq!(iter.next(), Some(&[20][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`split`]: slice::split
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Split<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    // Used for `SplitWhitespace` and `SplitAsciiWhitespace` `as_str` methods
    pub(crate) v: &'a [T],
    pred: P,
    // Used for `SplitAsciiWhitespace` `as_str` method
    pub(crate) finished: bool,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=as_slice | COMPLEXITY=10 | LINES=21

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> Split<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a [T], pred: P) -> Self {
        Self { v: slice, pred, finished: false }
    }
    /// Returns a slice which contains items not yet handled by split.
    /// # Example
    ///
    /// ```
    /// #![feature(split_as_slice)]
    /// let slice = [1,2,3,4,5];
    /// let mut split = slice.split(|v| v % 2 == 0);
    /// assert!(split.next().is_some());
    /// assert_eq!(split.as_slice(), &[3,4,5]);
    /// ```
    #[unstable(feature = "split_as_slice", issue = "96137")]
    pub fn as_slice(&self) -> &'a [T] {
        if self.finished { &[] } else { &self.v }
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for Split<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Split").field("v", &self.v).field("finished", &self.finished).finish()
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=11

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, P> Clone for Split<'_, T, P>
where
    P: Clone + FnMut(&T) -> bool,
{
    fn clone(&self) -> Self {
        Split { v: self.v, pred: self.pred.clone(), finished: self.finished }
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=next | COMPLEXITY=32 | LINES=39

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> Iterator for Split<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.finished {
            return None;
        }

        match self.v.iter().position(|x| (self.pred)(x)) {
            None => self.finish(),
            Some(idx) => {
                let (left, right) =
                    // SAFETY: if v.iter().position returns Some(idx), that
                    // idx is definitely a valid index for v
                    unsafe { (self.v.get_unchecked(..idx), self.v.get_unchecked(idx + 1..)) };
                let ret = Some(left);
                self.v = right;
                ret
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.finished {
            (0, Some(0))
        } else {
            // If the predicate doesn't match anything, we yield one slice.
            // If it matches every element, we yield `len() + 1` empty slices.
            (1, Some(self.v.len() + 1))
        }
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=23 | LINES=26

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> DoubleEndedIterator for Split<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.finished {
            return None;
        }

        match self.v.iter().rposition(|x| (self.pred)(x)) {
            None => self.finish(),
            Some(idx) => {
                let (left, right) =
                    // SAFETY: if v.iter().rposition returns Some(idx), then
                    // idx is definitely a valid index for v
                    unsafe { (self.v.get_unchecked(..idx), self.v.get_unchecked(idx + 1..)) };
                let ret = Some(right);
                self.v = left;
                ret
            }
        }
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=9 | LINES=15

```rust
impl<'a, T, P> SplitIter for Split<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn finish(&mut self) -> Option<&'a [T]> {
        if self.finished {
            None
        } else {
            self.finished = true;
            Some(self.v)
        }
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<T, P> FusedIterator for Split<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=SplitInclusive | COMPLEXITY=6 | LINES=29

```rust
/// An iterator over subslices separated by elements that match a predicate
/// function. Unlike `Split`, it contains the matched part as a terminator
/// of the subslice.
///
/// This struct is created by the [`split_inclusive`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = [10, 40, 33, 20];
/// let mut iter = slice.split_inclusive(|num| num % 3 == 0);
/// assert_eq!(iter.next(), Some(&[10, 40, 33][..]));
/// assert_eq!(iter.next(), Some(&[20][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`split_inclusive`]: slice::split_inclusive
/// [slices]: slice
#[stable(feature = "split_inclusive", since = "1.51.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SplitInclusive<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    v: &'a [T],
    pred: P,
    finished: bool,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitInclusive<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a [T], pred: P) -> Self {
        let finished = slice.is_empty();
        Self { v: slice, pred, finished }
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=13

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitInclusive<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitInclusive")
            .field("v", &self.v)
            .field("finished", &self.finished)
            .finish()
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=11

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<T, P> Clone for SplitInclusive<'_, T, P>
where
    P: Clone + FnMut(&T) -> bool,
{
    fn clone(&self) -> Self {
        SplitInclusive { v: self.v, pred: self.pred.clone(), finished: self.finished }
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=next | COMPLEXITY=20 | LINES=36

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<'a, T, P> Iterator for SplitInclusive<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.finished {
            return None;
        }

        let idx =
            self.v.iter().position(|x| (self.pred)(x)).map(|idx| idx + 1).unwrap_or(self.v.len());
        if idx == self.v.len() {
            self.finished = true;
        }
        let ret = Some(&self.v[..idx]);
        self.v = &self.v[idx..];
        ret
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.finished {
            (0, Some(0))
        } else {
            // If the predicate doesn't match anything, we yield one slice.
            // If it matches every element, we yield `len()` one-element slices,
            // or a single empty slice.
            (1, Some(cmp::max(1, self.v.len())))
        }
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=16 | LINES=25

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<'a, T, P> DoubleEndedIterator for SplitInclusive<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.finished {
            return None;
        }

        // The last index of self.v is already checked and found to match
        // by the last iteration, so we start searching a new match
        // one index to the left.
        let remainder = if self.v.is_empty() { &[] } else { &self.v[..(self.v.len() - 1)] };
        let idx = remainder.iter().rposition(|x| (self.pred)(x)).map(|idx| idx + 1).unwrap_or(0);
        if idx == 0 {
            self.finished = true;
        }
        let ret = Some(&self.v[idx..]);
        self.v = &self.v[..idx];
        ret
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<T, P> FusedIterator for SplitInclusive<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=STRUCT | NAME=SplitMut | COMPLEXITY=6 | LINES=25

```rust
/// An iterator over the mutable subslices of the vector which are separated
/// by elements that match `pred`.
///
/// This struct is created by the [`split_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut v = [10, 40, 30, 20, 60, 50];
/// let iter = v.split_mut(|num| *num % 3 == 0);
/// ```
///
/// [`split_mut`]: slice::split_mut
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SplitMut<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    v: &'a mut [T],
    pred: P,
    finished: bool,
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitMut<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a mut [T], pred: P) -> Self {
        Self { v: slice, pred, finished: false }
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitMut<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitMut").field("v", &self.v).field("finished", &self.finished).finish()
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=9 | LINES=15

```rust
impl<'a, T, P> SplitIter for SplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn finish(&mut self) -> Option<&'a mut [T]> {
        if self.finished {
            None
        } else {
            self.finished = true;
            Some(mem::take(&mut self.v))
        }
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=next | COMPLEXITY=23 | LINES=40

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> Iterator for SplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.finished {
            return None;
        }

        match self.v.iter().position(|x| (self.pred)(x)) {
            None => self.finish(),
            Some(idx) => {
                let tmp = mem::take(&mut self.v);
                // idx is the index of the element we are splitting on. We want to set self to the
                // region after idx, and return the subslice before and not including idx.
                // So first we split after idx
                let (head, tail) = tmp.split_at_mut(idx + 1);
                self.v = tail;
                // Then return the subslice up to but not including the found element
                Some(&mut head[..idx])
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.finished {
            (0, Some(0))
        } else {
            // If the predicate doesn't match anything, we yield one slice.
            // If it matches every element, we yield `len() + 1` empty slices.
            (1, Some(self.v.len() + 1))
        }
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=15 | LINES=28

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, P> DoubleEndedIterator for SplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.finished {
            return None;
        }

        let idx_opt = {
            // work around borrowck limitations
            let pred = &mut self.pred;
            self.v.iter().rposition(|x| (*pred)(x))
        };
        match idx_opt {
            None => self.finish(),
            Some(idx) => {
                let tmp = mem::take(&mut self.v);
                let (head, tail) = tmp.split_at_mut(idx);
                self.v = head;
                Some(&mut tail[1..])
            }
        }
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<T, P> FusedIterator for SplitMut<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=STRUCT | NAME=SplitInclusiveMut | COMPLEXITY=6 | LINES=26

```rust
/// An iterator over the mutable subslices of the vector which are separated
/// by elements that match `pred`. Unlike `SplitMut`, it contains the matched
/// parts in the ends of the subslices.
///
/// This struct is created by the [`split_inclusive_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut v = [10, 40, 30, 20, 60, 50];
/// let iter = v.split_inclusive_mut(|num| *num % 3 == 0);
/// ```
///
/// [`split_inclusive_mut`]: slice::split_inclusive_mut
/// [slices]: slice
#[stable(feature = "split_inclusive", since = "1.51.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SplitInclusiveMut<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    v: &'a mut [T],
    pred: P,
    finished: bool,
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitInclusiveMut<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a mut [T], pred: P) -> Self {
        let finished = slice.is_empty();
        Self { v: slice, pred, finished }
    }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=13

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitInclusiveMut<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitInclusiveMut")
            .field("v", &self.v)
            .field("finished", &self.finished)
            .finish()
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=next | COMPLEXITY=22 | LINES=41

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<'a, T, P> Iterator for SplitInclusiveMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.finished {
            return None;
        }

        let idx_opt = {
            // work around borrowck limitations
            let pred = &mut self.pred;
            self.v.iter().position(|x| (*pred)(x))
        };
        let idx = idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
        if idx == self.v.len() {
            self.finished = true;
        }
        let tmp = mem::take(&mut self.v);
        let (head, tail) = tmp.split_at_mut(idx);
        self.v = tail;
        Some(head)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.finished {
            (0, Some(0))
        } else {
            // If the predicate doesn't match anything, we yield one slice.
            // If it matches every element, we yield `len()` one-element slices,
            // or a single empty slice.
            (1, Some(cmp::max(1, self.v.len())))
        }
    }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=17 | LINES=34

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<'a, T, P> DoubleEndedIterator for SplitInclusiveMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.finished {
            return None;
        }

        let idx_opt = if self.v.is_empty() {
            None
        } else {
            // work around borrowck limitations
            let pred = &mut self.pred;

            // The last index of self.v is already checked and found to match
            // by the last iteration, so we start searching a new match
            // one index to the left.
            let remainder = &self.v[..(self.v.len() - 1)];
            remainder.iter().rposition(|x| (*pred)(x))
        };
        let idx = idx_opt.map(|idx| idx + 1).unwrap_or(0);
        if idx == 0 {
            self.finished = true;
        }
        let tmp = mem::take(&mut self.v);
        let (head, tail) = tmp.split_at_mut(idx);
        self.v = head;
        Some(tail)
    }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "split_inclusive", since = "1.51.0")]
impl<T, P> FusedIterator for SplitInclusiveMut<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=STRUCT | NAME=RSplit | COMPLEXITY=6 | LINES=26

```rust
/// An iterator over subslices separated by elements that match a predicate
/// function, starting from the end of the slice.
///
/// This struct is created by the [`rsplit`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = [11, 22, 33, 0, 44, 55];
/// let mut iter = slice.rsplit(|num| *num == 0);
/// assert_eq!(iter.next(), Some(&[44, 55][..]));
/// assert_eq!(iter.next(), Some(&[11, 22, 33][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`rsplit`]: slice::rsplit
/// [slices]: slice
#[stable(feature = "slice_rsplit", since = "1.27.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RSplit<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: Split<'a, T, P>,
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> RSplit<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a [T], pred: P) -> Self {
        Self { inner: Split::new(slice, pred) }
    }
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=13

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplit<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSplit")
            .field("v", &self.inner.v)
            .field("finished", &self.inner.finished)
            .finish()
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=11

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T, P> Clone for RSplit<'_, T, P>
where
    P: Clone + FnMut(&T) -> bool,
{
    fn clone(&self) -> Self {
        RSplit { inner: self.inner.clone() }
    }
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=next | COMPLEXITY=6 | LINES=18

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> Iterator for RSplit<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        self.inner.next_back()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> DoubleEndedIterator for RSplit<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        self.inner.next()
    }
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> SplitIter for RSplit<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn finish(&mut self) -> Option<&'a [T]> {
        self.inner.finish()
    }
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T, P> FusedIterator for RSplit<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=STRUCT | NAME=RSplitMut | COMPLEXITY=6 | LINES=23

```rust
/// An iterator over the subslices of the vector which are separated
/// by elements that match `pred`, starting from the end of the slice.
///
/// This struct is created by the [`rsplit_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = [11, 22, 33, 0, 44, 55];
/// let iter = slice.rsplit_mut(|num| *num == 0);
/// ```
///
/// [`rsplit_mut`]: slice::rsplit_mut
/// [slices]: slice
#[stable(feature = "slice_rsplit", since = "1.27.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RSplitMut<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: SplitMut<'a, T, P>,
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> RSplitMut<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a mut [T], pred: P) -> Self {
        Self { inner: SplitMut::new(slice, pred) }
    }
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=13

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitMut<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSplitMut")
            .field("v", &self.inner.v)
            .field("finished", &self.inner.finished)
            .finish()
    }
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=FUNCTION | NAME=finish | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> SplitIter for RSplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn finish(&mut self) -> Option<&'a mut [T]> {
        self.inner.finish()
    }
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=FUNCTION | NAME=next | COMPLEXITY=6 | LINES=18

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> Iterator for RSplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        self.inner.next_back()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<'a, T, P> DoubleEndedIterator for RSplitMut<'a, T, P>
where
    P: FnMut(&T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        self.inner.next()
    }
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "slice_rsplit", since = "1.27.0")]
impl<T, P> FusedIterator for RSplitMut<'_, T, P> where P: FnMut(&T) -> bool {}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=STRUCT | NAME=GenericSplitN | COMPLEXITY=5 | LINES=9

```rust
/// An private iterator over subslices separated by elements that
/// match a predicate function, splitting at most a fixed number of
/// times.
#[derive(Debug)]
struct GenericSplitN<I> {
    iter: I,
    count: usize,
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=FUNCTION | NAME=next | COMPLEXITY=13 | LINES=28

```rust
impl<T, I: SplitIter<Item = T>> Iterator for GenericSplitN<I> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match self.count {
            0 => None,
            1 => {
                self.count -= 1;
                self.iter.finish()
            }
            _ => {
                self.count -= 1;
                self.iter.next()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper_opt) = self.iter.size_hint();
        (
            cmp::min(self.count, lower),
            Some(upper_opt.map_or(self.count, |upper| cmp::min(self.count, upper))),
        )
    }
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=STRUCT | NAME=SplitN | COMPLEXITY=6 | LINES=26

```rust
/// An iterator over subslices separated by elements that match a predicate
/// function, limited to a given number of splits.
///
/// This struct is created by the [`splitn`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = [10, 40, 30, 20, 60, 50];
/// let mut iter = slice.splitn(2, |num| *num % 3 == 0);
/// assert_eq!(iter.next(), Some(&[10, 40][..]));
/// assert_eq!(iter.next(), Some(&[20, 60, 50][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`splitn`]: slice::splitn
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SplitN<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: GenericSplitN<Split<'a, T, P>>,
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitN<'a, T, P> {
    #[inline]
    pub(super) fn new(s: Split<'a, T, P>, n: usize) -> Self {
        Self { inner: GenericSplitN { iter: s, count: n } }
    }
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitN<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitN").field("inner", &self.inner).finish()
    }
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=STRUCT | NAME=RSplitN | COMPLEXITY=6 | LINES=27

```rust
/// An iterator over subslices separated by elements that match a
/// predicate function, limited to a given number of splits, starting
/// from the end of the slice.
///
/// This struct is created by the [`rsplitn`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = [10, 40, 30, 20, 60, 50];
/// let mut iter = slice.rsplitn(2, |num| *num % 3 == 0);
/// assert_eq!(iter.next(), Some(&[50][..]));
/// assert_eq!(iter.next(), Some(&[10, 40, 30, 20][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`rsplitn`]: slice::rsplitn
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RSplitN<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: GenericSplitN<RSplit<'a, T, P>>,
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> RSplitN<'a, T, P> {
    #[inline]
    pub(super) fn new(s: RSplit<'a, T, P>, n: usize) -> Self {
        Self { inner: GenericSplitN { iter: s, count: n } }
    }
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitN<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSplitN").field("inner", &self.inner).finish()
    }
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=STRUCT | NAME=SplitNMut | COMPLEXITY=6 | LINES=23

```rust
/// An iterator over subslices separated by elements that match a predicate
/// function, limited to a given number of splits.
///
/// This struct is created by the [`splitn_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = [10, 40, 30, 20, 60, 50];
/// let iter = slice.splitn_mut(2, |num| *num % 3 == 0);
/// ```
///
/// [`splitn_mut`]: slice::splitn_mut
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SplitNMut<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: GenericSplitN<SplitMut<'a, T, P>>,
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitNMut<'a, T, P> {
    #[inline]
    pub(super) fn new(s: SplitMut<'a, T, P>, n: usize) -> Self {
        Self { inner: GenericSplitN { iter: s, count: n } }
    }
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for SplitNMut<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitNMut").field("inner", &self.inner).finish()
    }
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=STRUCT | NAME=RSplitNMut | COMPLEXITY=6 | LINES=24

```rust
/// An iterator over subslices separated by elements that match a
/// predicate function, limited to a given number of splits, starting
/// from the end of the slice.
///
/// This struct is created by the [`rsplitn_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = [10, 40, 30, 20, 60, 50];
/// let iter = slice.rsplitn_mut(2, |num| *num % 3 == 0);
/// ```
///
/// [`rsplitn_mut`]: slice::rsplitn_mut
/// [slices]: slice
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RSplitNMut<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    inner: GenericSplitN<RSplitMut<'a, T, P>>,
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
impl<'a, T: 'a, P: FnMut(&T) -> bool> RSplitNMut<'a, T, P> {
    #[inline]
    pub(super) fn new(s: RSplitMut<'a, T, P>, n: usize) -> Self {
        Self { inner: GenericSplitN { iter: s, count: n } }
    }
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: fmt::Debug, P> fmt::Debug for RSplitNMut<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSplitNMut").field("inner", &self.inner).finish()
    }
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
forward_iterator! { SplitN: T, &'a [T] }
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
forward_iterator! { RSplitN: T, &'a [T] }
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
forward_iterator! { SplitNMut: T, &'a mut [T] }
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
forward_iterator! { RSplitNMut: T, &'a mut [T] }
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=STRUCT | NAME=Windows | COMPLEXITY=3 | LINES=25

```rust
/// An iterator over overlapping subslices of length `size`.
///
/// This struct is created by the [`windows`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = ['r', 'u', 's', 't'];
/// let mut iter = slice.windows(2);
/// assert_eq!(iter.next(), Some(&['r', 'u'][..]));
/// assert_eq!(iter.next(), Some(&['u', 's'][..]));
/// assert_eq!(iter.next(), Some(&['s', 't'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`windows`]: slice::windows
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windows<'a, T: 'a> {
    v: &'a [T],
    size: NonZero<usize>,
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a> Windows<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T], size: NonZero<usize>) -> Self {
        Self { v: slice, size }
    }
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Windows<'_, T> {
    fn clone(&self) -> Self {
        Windows { v: self.v, size: self.size }
    }
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=FUNCTION | NAME=next | COMPLEXITY=40 | LINES=64

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.size.get()]);
            self.v = &self.v[1..];
            ret
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size.get() > self.v.len() {
            (0, Some(0))
        } else {
            let size = self.v.len() - self.size.get() + 1;
            (size, Some(size))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let size = self.size.get();
        if let Some(rest) = self.v.get(n..)
            && let Some(nth) = rest.get(..size)
        {
            self.v = &rest[1..];
            Some(nth)
        } else {
            // setting length to 0 is cheaper than overwriting the pointer when assigning &[]
            self.v = &self.v[..0]; // cheaper than &[]
            None
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let start = self.v.len() - self.size.get();
            Some(&self.v[start..])
        }
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        // SAFETY: since the caller guarantees that `i` is in bounds,
        // which means that `i` cannot overflow an `isize`, and the
        // slice created by `from_raw_parts` is a subslice of `self.v`
        // thus is guaranteed to be valid for the lifetime `'a` of `self.v`.
        unsafe { from_raw_parts(self.v.as_ptr().add(idx), self.size.get()) }
    }
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=15 | LINES=27

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Windows<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[self.v.len() - self.size.get()..]);
            self.v = &self.v[..self.v.len() - 1];
            ret
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.v.len().overflowing_sub(n);
        if end < self.size.get() || overflow {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            let ret = &self.v[end - self.size.get()..end];
            self.v = &self.v[..end - 1];
            Some(ret)
        }
    }
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Windows<'_, T> {}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for Windows<'_, T> {}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Windows<'_, T> {}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for Windows<'a, T> {}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for Windows<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=STRUCT | NAME=Chunks | COMPLEXITY=3 | LINES=29

```rust
/// An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a
/// time), starting at the beginning of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last slice
/// of the iteration will be the remainder.
///
/// This struct is created by the [`chunks`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let mut iter = slice.chunks(2);
/// assert_eq!(iter.next(), Some(&['l', 'o'][..]));
/// assert_eq!(iter.next(), Some(&['r', 'e'][..]));
/// assert_eq!(iter.next(), Some(&['m'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`chunks`]: slice::chunks
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Chunks<'a, T: 'a> {
    v: &'a [T],
    chunk_size: usize,
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a> Chunks<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T], size: usize) -> Self {
        Self { v: slice, chunk_size: size }
    }
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Chunks<'_, T> {
    fn clone(&self) -> Self {
        Chunks { v: self.v, chunk_size: self.chunk_size }
    }
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=FUNCTION | NAME=next | COMPLEXITY=45 | LINES=74

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let chunksz = cmp::min(self.v.len(), self.chunk_size);
            let (fst, snd) = self.v.split_at(chunksz);
            self.v = snd;
            Some(fst)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.v.is_empty() {
            (0, Some(0))
        } else {
            let n = self.v.len() / self.chunk_size;
            let rem = self.v.len() % self.chunk_size;
            let n = if rem > 0 { n + 1 } else { n };
            (n, Some(n))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.chunk_size);
        // min(len) makes a wrong start harmless, but enables optimizing this to brachless code
        let chunk_start = &self.v[start.min(self.v.len())..];
        let (nth, remainder) = chunk_start.split_at(self.chunk_size.min(chunk_start.len()));
        if !overflow && start < self.v.len() {
            self.v = remainder;
            Some(nth)
        } else {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let start = (self.v.len() - 1) / self.chunk_size * self.chunk_size;
            Some(&self.v[start..])
        }
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let start = idx * self.chunk_size;
        // SAFETY: the caller guarantees that `i` is in bounds,
        // which means that `start` must be in bounds of the
        // underlying `self.v` slice, and we made sure that `len`
        // is also in bounds of `self.v`. Thus, `start` cannot overflow
        // an `isize`, and the slice constructed by `from_raw_parts`
        // is a subslice of `self.v` which is guaranteed to be valid
        // for the lifetime `'a` of `self.v`.
        unsafe {
            let len = cmp::min(self.v.len().unchecked_sub(start), self.chunk_size);
            from_raw_parts(self.v.as_ptr().add(start), len)
        }
    }
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=31 | LINES=48

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Chunks<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let remainder = self.v.len() % self.chunk_size;
            let chunksz = if remainder != 0 { remainder } else { self.chunk_size };
            // SAFETY: split_at_unchecked requires the argument be less than or
            // equal to the length. This is guaranteed, but subtle: `chunksz`
            // will always either be `self.v.len() % self.chunk_size`, which
            // will always evaluate to strictly less than `self.v.len()` (or
            // panic, in the case that `self.chunk_size` is zero), or it can be
            // `self.chunk_size`, in the case that the length is exactly
            // divisible by the chunk size.
            //
            // While it seems like using `self.chunk_size` in this case could
            // lead to a value greater than `self.v.len()`, it cannot: if
            // `self.chunk_size` were greater than `self.v.len()`, then
            // `self.v.len() % self.chunk_size` would return nonzero (note that
            // in this branch of the `if`, we already know that `self.v` is
            // non-empty).
            let (fst, snd) = unsafe { self.v.split_at_unchecked(self.v.len() - chunksz) };
            self.v = fst;
            Some(snd)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            let start = (len - 1 - n) * self.chunk_size;
            let end = match start.checked_add(self.chunk_size) {
                Some(res) => cmp::min(self.v.len(), res),
                None => self.v.len(),
            };
            let nth_back = &self.v[start..end];
            self.v = &self.v[..start];
            Some(nth_back)
        }
    }
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Chunks<'_, T> {}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for Chunks<'_, T> {}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Chunks<'_, T> {}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for Chunks<'a, T> {}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for Chunks<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=STRUCT | NAME=ChunksMut | COMPLEXITY=6 | LINES=32

```rust
/// An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size`
/// elements at a time), starting at the beginning of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last slice
/// of the iteration will be the remainder.
///
/// This struct is created by the [`chunks_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = ['l', 'o', 'r', 'e', 'm'];
/// let iter = slice.chunks_mut(2);
/// ```
///
/// [`chunks_mut`]: slice::chunks_mut
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunksMut<'a, T: 'a> {
    /// # Safety
    /// This slice pointer must point at a valid region of `T` with at least length `v.len()`. Normally,
    /// those requirements would mean that we could instead use a `&mut [T]` here, but we cannot
    /// because `__iterator_get_unchecked` needs to return `&mut [T]`, which guarantees certain aliasing
    /// properties that we cannot uphold if we hold on to the full original `&mut [T]`. Wrapping a raw
    /// slice instead lets us hand out non-overlapping `&mut [T]` subslices of the slice we wrap.
    v: *mut [T],
    chunk_size: usize,
    _marker: PhantomData<&'a mut T>,
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a> ChunksMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T], size: usize) -> Self {
        Self { v: slice, chunk_size: size, _marker: PhantomData }
    }
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=FUNCTION | NAME=next | COMPLEXITY=82 | LINES=82

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for ChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let sz = cmp::min(self.v.len(), self.chunk_size);
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, tail) = unsafe { self.v.split_at_mut(sz) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *head })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.v.is_empty() {
            (0, Some(0))
        } else {
            let n = self.v.len() / self.chunk_size;
            let rem = self.v.len() % self.chunk_size;
            let n = if rem > 0 { n + 1 } else { n };
            (n, Some(n))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
        let (start, overflow) = n.overflowing_mul(self.chunk_size);
        if start >= self.v.len() || overflow {
            self.v = &mut [];
            None
        } else {
            let end = match start.checked_add(self.chunk_size) {
                Some(sum) => cmp::min(self.v.len(), sum),
                None => self.v.len(),
            };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, tail) = unsafe { self.v.split_at_mut(end) };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (_, nth) = unsafe { head.split_at_mut(start) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth })
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let start = (self.v.len() - 1) / self.chunk_size * self.chunk_size;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *self.v.get_unchecked_mut(start..) })
        }
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let start = idx * self.chunk_size;
        // SAFETY: see comments for `Chunks::__iterator_get_unchecked` and `self.v`.
        //
        // Also note that the caller also guarantees that we're never called
        // with the same index again, and that no other methods that will
        // access this subslice are called, so it is valid for the returned
        // slice to be mutable.
        unsafe {
            let len = cmp::min(self.v.len().unchecked_sub(start), self.chunk_size);
            from_raw_parts_mut(self.v.as_mut_ptr().add(start), len)
        }
    }
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=50 | LINES=41

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for ChunksMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let remainder = self.v.len() % self.chunk_size;
            let sz = if remainder != 0 { remainder } else { self.chunk_size };
            let len = self.v.len();
            // SAFETY: Similar to `Chunks::next_back`
            let (head, tail) = unsafe { self.v.split_at_mut_unchecked(len - sz) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *tail })
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &mut [];
            None
        } else {
            let start = (len - 1 - n) * self.chunk_size;
            let end = match start.checked_add(self.chunk_size) {
                Some(res) => cmp::min(self.v.len(), res),
                None => self.v.len(),
            };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (temp, _tail) = unsafe { self.v.split_at_mut(end) };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, nth_back) = unsafe { temp.split_at_mut(start) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth_back })
        }
    }
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for ChunksMut<'_, T> {}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksMut<'_, T> {}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for ChunksMut<'_, T> {}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for ChunksMut<'a, T> {}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for ChunksMut<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Send for ChunksMut<'_, T> where T: Send {}
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Sync for ChunksMut<'_, T> where T: Sync {}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=STRUCT | NAME=ChunksExact | COMPLEXITY=4 | LINES=31

```rust
/// An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a
/// time), starting at the beginning of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last
/// up to `chunk_size-1` elements will be omitted but can be retrieved from
/// the [`remainder`] function from the iterator.
///
/// This struct is created by the [`chunks_exact`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let mut iter = slice.chunks_exact(2);
/// assert_eq!(iter.next(), Some(&['l', 'o'][..]));
/// assert_eq!(iter.next(), Some(&['r', 'e'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`chunks_exact`]: slice::chunks_exact
/// [`remainder`]: ChunksExact::remainder
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunksExact<'a, T: 'a> {
    v: &'a [T],
    rem: &'a [T],
    chunk_size: usize,
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=FUNCTION | NAME=remainder | COMPLEXITY=12 | LINES=34

```rust
impl<'a, T> ChunksExact<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T], chunk_size: usize) -> Self {
        let rem = slice.len() % chunk_size;
        let fst_len = slice.len() - rem;
        // SAFETY: 0 <= fst_len <= slice.len() by construction above
        let (fst, snd) = unsafe { slice.split_at_unchecked(fst_len) };
        Self { v: fst, rem: snd, chunk_size }
    }

    /// Returns the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    ///
    /// # Example
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.chunks_exact(2);
    /// assert_eq!(iter.remainder(), &['m'][..]);
    /// assert_eq!(iter.next(), Some(&['l', 'o'][..]));
    /// assert_eq!(iter.remainder(), &['m'][..]);
    /// assert_eq!(iter.next(), Some(&['r', 'e'][..]));
    /// assert_eq!(iter.remainder(), &['m'][..]);
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remainder(), &['m'][..]);
    /// ```
    #[must_use]
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    pub fn remainder(&self) -> &'a [T] {
        self.rem
    }
}
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> Clone for ChunksExact<'_, T> {
    fn clone(&self) -> Self {
        ChunksExact { v: self.v, rem: self.rem, chunk_size: self.chunk_size }
    }
}
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=FUNCTION | NAME=next | COMPLEXITY=29 | LINES=51

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> Iterator for ChunksExact<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.chunk_size);
            self.v = snd;
            Some(fst)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.v.len() / self.chunk_size;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.chunk_size);
        if start >= self.v.len() || overflow {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            let (_, snd) = self.v.split_at(start);
            self.v = snd;
            self.next()
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let start = idx * self.chunk_size;
        // SAFETY: mostly identical to `Chunks::__iterator_get_unchecked`.
        unsafe { from_raw_parts(self.v.as_ptr().add(start), self.chunk_size) }
    }
}
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=15 | LINES=29

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for ChunksExact<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.v.len() - self.chunk_size);
            self.v = fst;
            Some(snd)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            let start = (len - 1 - n) * self.chunk_size;
            let end = start + self.chunk_size;
            let nth_back = &self.v[start..end];
            self.v = &self.v[..start];
            Some(nth_back)
        }
    }
}
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> ExactSizeIterator for ChunksExact<'_, T> {
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksExact<'_, T> {}
```

## Block 128
**Metadata**: AST_ID=128 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> FusedIterator for ChunksExact<'_, T> {}
```

## Block 129
**Metadata**: AST_ID=129 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for ChunksExact<'a, T> {}
```

## Block 130
**Metadata**: AST_ID=130 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for ChunksExact<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 131
**Metadata**: AST_ID=131 | TYPE=STRUCT | NAME=ChunksExactMut | COMPLEXITY=7 | LINES=35

```rust
/// An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size`
/// elements at a time), starting at the beginning of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last up to
/// `chunk_size-1` elements will be omitted but can be retrieved from the
/// [`into_remainder`] function from the iterator.
///
/// This struct is created by the [`chunks_exact_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = ['l', 'o', 'r', 'e', 'm'];
/// let iter = slice.chunks_exact_mut(2);
/// ```
///
/// [`chunks_exact_mut`]: slice::chunks_exact_mut
/// [`into_remainder`]: ChunksExactMut::into_remainder
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "chunks_exact", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunksExactMut<'a, T: 'a> {
    /// # Safety
    /// This slice pointer must point at a valid region of `T` with at least length `v.len()`. Normally,
    /// those requirements would mean that we could instead use a `&mut [T]` here, but we cannot
    /// because `__iterator_get_unchecked` needs to return `&mut [T]`, which guarantees certain aliasing
    /// properties that we cannot uphold if we hold on to the full original `&mut [T]`. Wrapping a raw
    /// slice instead lets us hand out non-overlapping `&mut [T]` subslices of the slice we wrap.
    v: *mut [T],
    rem: &'a mut [T], // The iterator never yields from here, so this can be unique
    chunk_size: usize,
    _marker: PhantomData<&'a mut T>,
}
```

## Block 132
**Metadata**: AST_ID=132 | TYPE=FUNCTION | NAME=into_remainder | COMPLEXITY=13 | LINES=20

```rust
impl<'a, T> ChunksExactMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T], chunk_size: usize) -> Self {
        let rem = slice.len() % chunk_size;
        let fst_len = slice.len() - rem;
        // SAFETY: 0 <= fst_len <= slice.len() by construction above
        let (fst, snd) = unsafe { slice.split_at_mut_unchecked(fst_len) };
        Self { v: fst, rem: snd, chunk_size, _marker: PhantomData }
    }

    /// Returns the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    pub fn into_remainder(self) -> &'a mut [T] {
        self.rem
    }
}
```

## Block 133
**Metadata**: AST_ID=133 | TYPE=FUNCTION | NAME=next | COMPLEXITY=47 | LINES=54

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> Iterator for ChunksExactMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            // SAFETY: self.chunk_size is inbounds because we compared above against self.v.len()
            let (head, tail) = unsafe { self.v.split_at_mut(self.chunk_size) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *head })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.v.len() / self.chunk_size;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
        let (start, overflow) = n.overflowing_mul(self.chunk_size);
        if start >= self.v.len() || overflow {
            self.v = &mut [];
            None
        } else {
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (_, snd) = unsafe { self.v.split_at_mut(start) };
            self.v = snd;
            self.next()
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let start = idx * self.chunk_size;
        // SAFETY: see comments for `Chunks::__iterator_get_unchecked` and `self.v`.
        unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), self.chunk_size) }
    }
}
```

## Block 134
**Metadata**: AST_ID=134 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=41 | LINES=35

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for ChunksExactMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            // SAFETY: This subtraction is inbounds because of the check above
            let (head, tail) = unsafe { self.v.split_at_mut(self.v.len() - self.chunk_size) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *tail })
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &mut [];
            None
        } else {
            let start = (len - 1 - n) * self.chunk_size;
            let end = start + self.chunk_size;
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (temp, _tail) = unsafe { mem::replace(&mut self.v, &mut []).split_at_mut(end) };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, nth_back) = unsafe { temp.split_at_mut(start) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth_back })
        }
    }
}
```

## Block 135
**Metadata**: AST_ID=135 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> ExactSizeIterator for ChunksExactMut<'_, T> {
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
```

## Block 136
**Metadata**: AST_ID=136 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for ChunksExactMut<'_, T> {}
```

## Block 137
**Metadata**: AST_ID=137 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
impl<T> FusedIterator for ChunksExactMut<'_, T> {}
```

## Block 138
**Metadata**: AST_ID=138 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for ChunksExactMut<'a, T> {}
```

## Block 139
**Metadata**: AST_ID=139 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for ChunksExactMut<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 140
**Metadata**: AST_ID=140 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
unsafe impl<T> Send for ChunksExactMut<'_, T> where T: Send {}
```

## Block 141
**Metadata**: AST_ID=141 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "chunks_exact", since = "1.31.0")]
unsafe impl<T> Sync for ChunksExactMut<'_, T> where T: Sync {}
```

## Block 142
**Metadata**: AST_ID=142 | TYPE=STRUCT | NAME=ArrayWindows | COMPLEXITY=3 | LINES=29

```rust
/// A windowed iterator over a slice in overlapping chunks (`N` elements at a
/// time), starting at the beginning of the slice
///
/// This struct is created by the [`array_windows`] method on [slices].
///
/// # Example
///
/// ```
/// #![feature(array_windows)]
///
/// let slice = [0, 1, 2, 3];
/// let mut iter = slice.array_windows::<2>();
/// assert_eq!(iter.next(), Some(&[0, 1]));
/// assert_eq!(iter.next(), Some(&[1, 2]));
/// assert_eq!(iter.next(), Some(&[2, 3]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`array_windows`]: slice::array_windows
/// [slices]: slice
#[derive(Debug, Clone, Copy)]
#[unstable(feature = "array_windows", issue = "75027")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ArrayWindows<'a, T: 'a, const N: usize> {
    slice_head: *const T,
    num: usize,
    marker: PhantomData<&'a [T; N]>,
}
```

## Block 143
**Metadata**: AST_ID=143 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<'a, T: 'a, const N: usize> ArrayWindows<'a, T, N> {
    #[inline]
    pub(super) const fn new(slice: &'a [T]) -> Self {
        let num_windows = slice.len().saturating_sub(N - 1);
        Self { slice_head: slice.as_ptr(), num: num_windows, marker: PhantomData }
    }
}
```

## Block 144
**Metadata**: AST_ID=144 | TYPE=FUNCTION | NAME=next | COMPLEXITY=37 | LINES=52

```rust
#[unstable(feature = "array_windows", issue = "75027")]
impl<'a, T, const N: usize> Iterator for ArrayWindows<'a, T, N> {
    type Item = &'a [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            return None;
        }
        // SAFETY:
        // This is safe because it's indexing into a slice guaranteed to be length > N.
        let ret = unsafe { &*self.slice_head.cast::<[T; N]>() };
        // SAFETY: Guaranteed that there are at least 1 item remaining otherwise
        // earlier branch would've been hit
        self.slice_head = unsafe { self.slice_head.add(1) };

        self.num -= 1;
        Some(ret)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num, Some(self.num))
    }

    #[inline]
    fn count(self) -> usize {
        self.num
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.num <= n {
            self.num = 0;
            return None;
        }
        // SAFETY:
        // This is safe because it's indexing into a slice guaranteed to be length > N.
        let ret = unsafe { &*self.slice_head.add(n).cast::<[T; N]>() };
        // SAFETY: Guaranteed that there are at least n items remaining
        self.slice_head = unsafe { self.slice_head.add(n + 1) };

        self.num -= n + 1;
        Some(ret)
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.nth(self.num.checked_sub(1)?)
    }
}
```

## Block 145
**Metadata**: AST_ID=145 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=27 | LINES=26

```rust
#[unstable(feature = "array_windows", issue = "75027")]
impl<'a, T, const N: usize> DoubleEndedIterator for ArrayWindows<'a, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T; N]> {
        if self.num == 0 {
            return None;
        }
        // SAFETY: Guaranteed that there are n items remaining, n-1 for 0-indexing.
        let ret = unsafe { &*self.slice_head.add(self.num - 1).cast::<[T; N]>() };
        self.num -= 1;
        Some(ret)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<&'a [T; N]> {
        if self.num <= n {
            self.num = 0;
            return None;
        }
        // SAFETY: Guaranteed that there are n items remaining, n-1 for 0-indexing.
        let ret = unsafe { &*self.slice_head.add(self.num - (n + 1)).cast::<[T; N]>() };
        self.num -= n + 1;
        Some(ret)
    }
}
```

## Block 146
**Metadata**: AST_ID=146 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "array_windows", issue = "75027")]
impl<T, const N: usize> ExactSizeIterator for ArrayWindows<'_, T, N> {
    fn is_empty(&self) -> bool {
        self.num == 0
    }
}
```

## Block 147
**Metadata**: AST_ID=147 | TYPE=STRUCT | NAME=RChunks | COMPLEXITY=3 | LINES=29

```rust
/// An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a
/// time), starting at the end of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last slice
/// of the iteration will be the remainder.
///
/// This struct is created by the [`rchunks`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let mut iter = slice.rchunks(2);
/// assert_eq!(iter.next(), Some(&['e', 'm'][..]));
/// assert_eq!(iter.next(), Some(&['o', 'r'][..]));
/// assert_eq!(iter.next(), Some(&['l'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`rchunks`]: slice::rchunks
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RChunks<'a, T: 'a> {
    v: &'a [T],
    chunk_size: usize,
}
```

## Block 148
**Metadata**: AST_ID=148 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a> RChunks<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T], size: usize) -> Self {
        Self { v: slice, chunk_size: size }
    }
}
```

## Block 149
**Metadata**: AST_ID=149 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> Clone for RChunks<'_, T> {
    fn clone(&self) -> Self {
        RChunks { v: self.v, chunk_size: self.chunk_size }
    }
}
```

## Block 150
**Metadata**: AST_ID=150 | TYPE=FUNCTION | NAME=next | COMPLEXITY=65 | LINES=80

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let len = self.v.len();
            let chunksz = cmp::min(len, self.chunk_size);
            // SAFETY: split_at_unchecked just requires the argument be less
            // than the length. This could only happen if the expression `len -
            // chunksz` overflows. This could only happen if `chunksz > len`,
            // which is impossible as we initialize it as the `min` of `len` and
            // `self.chunk_size`.
            let (fst, snd) = unsafe { self.v.split_at_unchecked(len - chunksz) };
            self.v = fst;
            Some(snd)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.v.is_empty() {
            (0, Some(0))
        } else {
            let n = self.v.len() / self.chunk_size;
            let rem = self.v.len() % self.chunk_size;
            let n = if rem > 0 { n + 1 } else { n };
            (n, Some(n))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = n.overflowing_mul(self.chunk_size);
        if end >= self.v.len() || overflow {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            // Can't underflow because of the check above
            let end = self.v.len() - end;
            let start = match end.checked_sub(self.chunk_size) {
                Some(sum) => sum,
                None => 0,
            };
            let nth = &self.v[start..end];
            self.v = &self.v[0..start];
            Some(nth)
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let rem = self.v.len() % self.chunk_size;
            let end = if rem == 0 { self.chunk_size } else { rem };
            Some(&self.v[0..end])
        }
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let end = self.v.len() - idx * self.chunk_size;
        let start = match end.checked_sub(self.chunk_size) {
            None => 0,
            Some(start) => start,
        };
        // SAFETY: mostly identical to `Chunks::__iterator_get_unchecked`.
        unsafe { from_raw_parts(self.v.as_ptr().add(start), end - start) }
    }
}
```

## Block 151
**Metadata**: AST_ID=151 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=25 | LINES=34

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunks<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let remainder = self.v.len() % self.chunk_size;
            let chunksz = if remainder != 0 { remainder } else { self.chunk_size };
            // SAFETY: similar to Chunks::next_back
            let (fst, snd) = unsafe { self.v.split_at_unchecked(chunksz) };
            self.v = snd;
            Some(fst)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            // can't underflow because `n < len`
            let offset_from_end = (len - 1 - n) * self.chunk_size;
            let end = self.v.len() - offset_from_end;
            let start = end.saturating_sub(self.chunk_size);
            let nth_back = &self.v[start..end];
            self.v = &self.v[end..];
            Some(nth_back)
        }
    }
}
```

## Block 152
**Metadata**: AST_ID=152 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunks<'_, T> {}
```

## Block 153
**Metadata**: AST_ID=153 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunks<'_, T> {}
```

## Block 154
**Metadata**: AST_ID=154 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunks<'_, T> {}
```

## Block 155
**Metadata**: AST_ID=155 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for RChunks<'a, T> {}
```

## Block 156
**Metadata**: AST_ID=156 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for RChunks<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 157
**Metadata**: AST_ID=157 | TYPE=STRUCT | NAME=RChunksMut | COMPLEXITY=6 | LINES=32

```rust
/// An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size`
/// elements at a time), starting at the end of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last slice
/// of the iteration will be the remainder.
///
/// This struct is created by the [`rchunks_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = ['l', 'o', 'r', 'e', 'm'];
/// let iter = slice.rchunks_mut(2);
/// ```
///
/// [`rchunks_mut`]: slice::rchunks_mut
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RChunksMut<'a, T: 'a> {
    /// # Safety
    /// This slice pointer must point at a valid region of `T` with at least length `v.len()`. Normally,
    /// those requirements would mean that we could instead use a `&mut [T]` here, but we cannot
    /// because `__iterator_get_unchecked` needs to return `&mut [T]`, which guarantees certain aliasing
    /// properties that we cannot uphold if we hold on to the full original `&mut [T]`. Wrapping a raw
    /// slice instead lets us hand out non-overlapping `&mut [T]` subslices of the slice we wrap.
    v: *mut [T],
    chunk_size: usize,
    _marker: PhantomData<&'a mut T>,
}
```

## Block 158
**Metadata**: AST_ID=158 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
impl<'a, T: 'a> RChunksMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T], size: usize) -> Self {
        Self { v: slice, chunk_size: size, _marker: PhantomData }
    }
}
```

## Block 159
**Metadata**: AST_ID=159 | TYPE=FUNCTION | NAME=next | COMPLEXITY=93 | LINES=89

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let sz = cmp::min(self.v.len(), self.chunk_size);
            let len = self.v.len();
            // SAFETY: split_at_mut_unchecked just requires the argument be less
            // than the length. This could only happen if the expression
            // `len - sz` overflows. This could only happen if `sz >
            // len`, which is impossible as we initialize it as the `min` of
            // `self.v.len()` (e.g. `len`) and `self.chunk_size`.
            let (head, tail) = unsafe { self.v.split_at_mut_unchecked(len - sz) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *tail })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.v.is_empty() {
            (0, Some(0))
        } else {
            let n = self.v.len() / self.chunk_size;
            let rem = self.v.len() % self.chunk_size;
            let n = if rem > 0 { n + 1 } else { n };
            (n, Some(n))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
        let (end, overflow) = n.overflowing_mul(self.chunk_size);
        if end >= self.v.len() || overflow {
            self.v = &mut [];
            None
        } else {
            // Can't underflow because of the check above
            let end = self.v.len() - end;
            let start = match end.checked_sub(self.chunk_size) {
                Some(sum) => sum,
                None => 0,
            };
            // SAFETY: This type ensures that self.v is a valid pointer with a correct len.
            // Therefore the bounds check in split_at_mut guarantees the split point is inbounds.
            let (head, tail) = unsafe { self.v.split_at_mut(start) };
            // SAFETY: This type ensures that self.v is a valid pointer with a correct len.
            // Therefore the bounds check in split_at_mut guarantees the split point is inbounds.
            let (nth, _) = unsafe { tail.split_at_mut(end - start) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth })
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let rem = self.v.len() % self.chunk_size;
            let end = if rem == 0 { self.chunk_size } else { rem };
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *self.v.get_unchecked_mut(0..end) })
        }
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let end = self.v.len() - idx * self.chunk_size;
        let start = match end.checked_sub(self.chunk_size) {
            None => 0,
            Some(start) => start,
        };
        // SAFETY: see comments for `RChunks::__iterator_get_unchecked` and
        // `ChunksMut::__iterator_get_unchecked`, `self.v`.
        unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), end - start) }
    }
}
```

## Block 160
**Metadata**: AST_ID=160 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=46 | LINES=39

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let remainder = self.v.len() % self.chunk_size;
            let sz = if remainder != 0 { remainder } else { self.chunk_size };
            // SAFETY: Similar to `Chunks::next_back`
            let (head, tail) = unsafe { self.v.split_at_mut_unchecked(sz) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *head })
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &mut [];
            None
        } else {
            // can't underflow because `n < len`
            let offset_from_end = (len - 1 - n) * self.chunk_size;
            let end = self.v.len() - offset_from_end;
            let start = end.saturating_sub(self.chunk_size);
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (tmp, tail) = unsafe { self.v.split_at_mut(end) };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (_, nth_back) = unsafe { tmp.split_at_mut(start) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth_back })
        }
    }
}
```

## Block 161
**Metadata**: AST_ID=161 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunksMut<'_, T> {}
```

## Block 162
**Metadata**: AST_ID=162 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksMut<'_, T> {}
```

## Block 163
**Metadata**: AST_ID=163 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksMut<'_, T> {}
```

## Block 164
**Metadata**: AST_ID=164 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksMut<'a, T> {}
```

## Block 165
**Metadata**: AST_ID=165 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for RChunksMut<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 166
**Metadata**: AST_ID=166 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<T> Send for RChunksMut<'_, T> where T: Send {}
```

## Block 167
**Metadata**: AST_ID=167 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<T> Sync for RChunksMut<'_, T> where T: Sync {}
```

## Block 168
**Metadata**: AST_ID=168 | TYPE=STRUCT | NAME=RChunksExact | COMPLEXITY=4 | LINES=31

```rust
/// An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a
/// time), starting at the end of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last
/// up to `chunk_size-1` elements will be omitted but can be retrieved from
/// the [`remainder`] function from the iterator.
///
/// This struct is created by the [`rchunks_exact`] method on [slices].
///
/// # Example
///
/// ```
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let mut iter = slice.rchunks_exact(2);
/// assert_eq!(iter.next(), Some(&['e', 'm'][..]));
/// assert_eq!(iter.next(), Some(&['o', 'r'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`rchunks_exact`]: slice::rchunks_exact
/// [`remainder`]: RChunksExact::remainder
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RChunksExact<'a, T: 'a> {
    v: &'a [T],
    rem: &'a [T],
    chunk_size: usize,
}
```

## Block 169
**Metadata**: AST_ID=169 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=12 | LINES=34

```rust
impl<'a, T> RChunksExact<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T], chunk_size: usize) -> Self {
        let rem = slice.len() % chunk_size;
        // SAFETY: 0 <= rem <= slice.len() by construction above
        let (fst, snd) = unsafe { slice.split_at_unchecked(rem) };
        Self { v: snd, rem: fst, chunk_size }
    }

    /// Returns the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    ///
    /// # Example
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.rchunks_exact(2);
    /// assert_eq!(iter.remainder(), &['l'][..]);
    /// assert_eq!(iter.next(), Some(&['e', 'm'][..]));
    /// assert_eq!(iter.remainder(), &['l'][..]);
    /// assert_eq!(iter.next(), Some(&['o', 'r'][..]));
    /// assert_eq!(iter.remainder(), &['l'][..]);
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remainder(), &['l'][..]);
    /// ```
    #[must_use]
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[rustc_const_unstable(feature = "const_slice_make_iter", issue = "137737")]
    pub const fn remainder(&self) -> &'a [T] {
        self.rem
    }
}
```

## Block 170
**Metadata**: AST_ID=170 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=8

```rust
// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Clone for RChunksExact<'a, T> {
    fn clone(&self) -> RChunksExact<'a, T> {
        RChunksExact { v: self.v, rem: self.rem, chunk_size: self.chunk_size }
    }
}
```

## Block 171
**Metadata**: AST_ID=171 | TYPE=FUNCTION | NAME=next | COMPLEXITY=29 | LINES=52

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksExact<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.v.len() - self.chunk_size);
            self.v = fst;
            Some(snd)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.v.len() / self.chunk_size;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = n.overflowing_mul(self.chunk_size);
        if end >= self.v.len() || overflow {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            let (fst, _) = self.v.split_at(self.v.len() - end);
            self.v = fst;
            self.next()
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let end = self.v.len() - idx * self.chunk_size;
        let start = end - self.chunk_size;
        // SAFETY: mostly identical to `Chunks::__iterator_get_unchecked`.
        unsafe { from_raw_parts(self.v.as_ptr().add(start), self.chunk_size) }
    }
}
```

## Block 172
**Metadata**: AST_ID=172 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=15 | LINES=32

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksExact<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.chunk_size);
            self.v = snd;
            Some(fst)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &self.v[..0]; // cheaper than &[]
            None
        } else {
            // now that we know that `n` corresponds to a chunk,
            // none of these operations can underflow/overflow
            let offset = (len - n) * self.chunk_size;
            let start = self.v.len() - offset;
            let end = start + self.chunk_size;
            let nth_back = &self.v[start..end];
            self.v = &self.v[end..];
            Some(nth_back)
        }
    }
}
```

## Block 173
**Metadata**: AST_ID=173 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> ExactSizeIterator for RChunksExact<'a, T> {
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
```

## Block 174
**Metadata**: AST_ID=174 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksExact<'_, T> {}
```

## Block 175
**Metadata**: AST_ID=175 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksExact<'_, T> {}
```

## Block 176
**Metadata**: AST_ID=176 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksExact<'a, T> {}
```

## Block 177
**Metadata**: AST_ID=177 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for RChunksExact<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 178
**Metadata**: AST_ID=178 | TYPE=STRUCT | NAME=RChunksExactMut | COMPLEXITY=6 | LINES=34

```rust
/// An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size`
/// elements at a time), starting at the end of the slice.
///
/// When the slice len is not evenly divided by the chunk size, the last up to
/// `chunk_size-1` elements will be omitted but can be retrieved from the
/// [`into_remainder`] function from the iterator.
///
/// This struct is created by the [`rchunks_exact_mut`] method on [slices].
///
/// # Example
///
/// ```
/// let mut slice = ['l', 'o', 'r', 'e', 'm'];
/// let iter = slice.rchunks_exact_mut(2);
/// ```
///
/// [`rchunks_exact_mut`]: slice::rchunks_exact_mut
/// [`into_remainder`]: RChunksExactMut::into_remainder
/// [slices]: slice
#[derive(Debug)]
#[stable(feature = "rchunks", since = "1.31.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RChunksExactMut<'a, T: 'a> {
    /// # Safety
    /// This slice pointer must point at a valid region of `T` with at least length `v.len()`. Normally,
    /// those requirements would mean that we could instead use a `&mut [T]` here, but we cannot
    /// because `__iterator_get_unchecked` needs to return `&mut [T]`, which guarantees certain aliasing
    /// properties that we cannot uphold if we hold on to the full original `&mut [T]`. Wrapping a raw
    /// slice instead lets us hand out non-overlapping `&mut [T]` subslices of the slice we wrap.
    v: *mut [T],
    rem: &'a mut [T],
    chunk_size: usize,
}
```

## Block 179
**Metadata**: AST_ID=179 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=13 | LINES=20

```rust
impl<'a, T> RChunksExactMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T], chunk_size: usize) -> Self {
        let rem = slice.len() % chunk_size;
        // SAFETY: 0 <= rem <= slice.len() by construction above
        let (fst, snd) = unsafe { slice.split_at_mut_unchecked(rem) };
        Self { v: snd, rem: fst, chunk_size }
    }

    /// Returns the remainder of the original slice that is not going to be
    /// returned by the iterator. The returned slice has at most `chunk_size-1`
    /// elements.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[rustc_const_unstable(feature = "const_slice_make_iter", issue = "137737")]
    pub const fn into_remainder(self) -> &'a mut [T] {
        self.rem
    }
}
```

## Block 180
**Metadata**: AST_ID=180 | TYPE=FUNCTION | NAME=next | COMPLEXITY=47 | LINES=57

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> Iterator for RChunksExactMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            let len = self.v.len();
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, tail) = unsafe { self.v.split_at_mut(len - self.chunk_size) };
            self.v = head;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *tail })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.v.len() / self.chunk_size;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a mut [T]> {
        let (end, overflow) = n.overflowing_mul(self.chunk_size);
        if end >= self.v.len() || overflow {
            self.v = &mut [];
            None
        } else {
            let len = self.v.len();
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (fst, _) = unsafe { self.v.split_at_mut(len - end) };
            self.v = fst;
            self.next()
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item {
        let end = self.v.len() - idx * self.chunk_size;
        let start = end - self.chunk_size;
        // SAFETY: see comments for `RChunksMut::__iterator_get_unchecked` and `self.v`.
        unsafe { from_raw_parts_mut(self.v.as_mut_ptr().add(start), self.chunk_size) }
    }
}
```

## Block 181
**Metadata**: AST_ID=181 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=42 | LINES=38

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<'a, T> DoubleEndedIterator for RChunksExactMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut [T]> {
        if self.v.len() < self.chunk_size {
            None
        } else {
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (head, tail) = unsafe { self.v.split_at_mut(self.chunk_size) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *head })
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.v = &mut [];
            None
        } else {
            // now that we know that `n` corresponds to a chunk,
            // none of these operations can underflow/overflow
            let offset = (len - n) * self.chunk_size;
            let start = self.v.len() - offset;
            let end = start + self.chunk_size;
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (tmp, tail) = unsafe { self.v.split_at_mut(end) };
            // SAFETY: The self.v contract ensures that any split_at_mut is valid.
            let (_, nth_back) = unsafe { tmp.split_at_mut(start) };
            self.v = tail;
            // SAFETY: Nothing else points to or will point to the contents of this slice.
            Some(unsafe { &mut *nth_back })
        }
    }
}
```

## Block 182
**Metadata**: AST_ID=182 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> ExactSizeIterator for RChunksExactMut<'_, T> {
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}
```

## Block 183
**Metadata**: AST_ID=183 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for RChunksExactMut<'_, T> {}
```

## Block 184
**Metadata**: AST_ID=184 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
impl<T> FusedIterator for RChunksExactMut<'_, T> {}
```

## Block 185
**Metadata**: AST_ID=185 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for RChunksExactMut<'a, T> {}
```

## Block 186
**Metadata**: AST_ID=186 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for RChunksExactMut<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 187
**Metadata**: AST_ID=187 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<T> Send for RChunksExactMut<'_, T> where T: Send {}
```

## Block 188
**Metadata**: AST_ID=188 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[stable(feature = "rchunks", since = "1.31.0")]
unsafe impl<T> Sync for RChunksExactMut<'_, T> where T: Sync {}
```

## Block 189
**Metadata**: AST_ID=189 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for Iter<'a, T> {}
```

## Block 190
**Metadata**: AST_ID=190 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for Iter<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 191
**Metadata**: AST_ID=191 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccess for IterMut<'a, T> {}
```

## Block 192
**Metadata**: AST_ID=192 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<'a, T> TrustedRandomAccessNoCoerce for IterMut<'a, T> {
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}
```

## Block 193
**Metadata**: AST_ID=193 | TYPE=STRUCT | NAME=ChunkBy | COMPLEXITY=2 | LINES=13

```rust
/// An iterator over slice in (non-overlapping) chunks separated by a predicate.
///
/// This struct is created by the [`chunk_by`] method on [slices].
///
/// [`chunk_by`]: slice::chunk_by
/// [slices]: slice
#[stable(feature = "slice_group_by", since = "1.77.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunkBy<'a, T: 'a, P> {
    slice: &'a [T],
    predicate: P,
}
```

## Block 194
**Metadata**: AST_ID=194 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> ChunkBy<'a, T, P> {
    pub(super) const fn new(slice: &'a [T], predicate: P) -> Self {
        ChunkBy { slice, predicate }
    }
}
```

## Block 195
**Metadata**: AST_ID=195 | TYPE=FUNCTION | NAME=next | COMPLEXITY=23 | LINES=34

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> Iterator for ChunkBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let (head, tail) = self.slice.split_at(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() { (0, Some(0)) } else { (1, Some(self.slice.len())) }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
```

## Block 196
**Metadata**: AST_ID=196 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=17 | LINES=22

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> DoubleEndedIterator for ChunkBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let (head, tail) = self.slice.split_at(self.slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}
```

## Block 197
**Metadata**: AST_ID=197 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> FusedIterator for ChunkBy<'a, T, P> where P: FnMut(&T, &T) -> bool {}
```

## Block 198
**Metadata**: AST_ID=198 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=6 | LINES=7

```rust
#[stable(feature = "slice_group_by_clone", since = "1.89.0")]
impl<'a, T: 'a, P: Clone> Clone for ChunkBy<'a, T, P> {
    fn clone(&self) -> Self {
        Self { slice: self.slice, predicate: self.predicate.clone() }
    }
}
```

## Block 199
**Metadata**: AST_ID=199 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a + fmt::Debug, P> fmt::Debug for ChunkBy<'a, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkBy").field("slice", &self.slice).finish()
    }
}
```

## Block 200
**Metadata**: AST_ID=200 | TYPE=STRUCT | NAME=ChunkByMut | COMPLEXITY=2 | LINES=14

```rust
/// An iterator over slice in (non-overlapping) mutable chunks separated
/// by a predicate.
///
/// This struct is created by the [`chunk_by_mut`] method on [slices].
///
/// [`chunk_by_mut`]: slice::chunk_by_mut
/// [slices]: slice
#[stable(feature = "slice_group_by", since = "1.77.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunkByMut<'a, T: 'a, P> {
    slice: &'a mut [T],
    predicate: P,
}
```

## Block 201
**Metadata**: AST_ID=201 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> ChunkByMut<'a, T, P> {
    pub(super) const fn new(slice: &'a mut [T], predicate: P) -> Self {
        ChunkByMut { slice, predicate }
    }
}
```

## Block 202
**Metadata**: AST_ID=202 | TYPE=FUNCTION | NAME=next | COMPLEXITY=23 | LINES=35

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> Iterator for ChunkByMut<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let slice = mem::take(&mut self.slice);
            let (head, tail) = slice.split_at_mut(len);
            self.slice = tail;
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.slice.is_empty() { (0, Some(0)) } else { (1, Some(self.slice.len())) }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
```

## Block 203
**Metadata**: AST_ID=203 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=17 | LINES=23

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> DoubleEndedIterator for ChunkByMut<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            None
        } else {
            let mut len = 1;
            let mut iter = self.slice.windows(2);
            while let Some([l, r]) = iter.next_back() {
                if (self.predicate)(l, r) { len += 1 } else { break }
            }
            let slice = mem::take(&mut self.slice);
            let (head, tail) = slice.split_at_mut(slice.len() - len);
            self.slice = head;
            Some(tail)
        }
    }
}
```

## Block 204
**Metadata**: AST_ID=204 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a, P> FusedIterator for ChunkByMut<'a, T, P> where P: FnMut(&T, &T) -> bool {}
```

## Block 205
**Metadata**: AST_ID=205 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "slice_group_by", since = "1.77.0")]
impl<'a, T: 'a + fmt::Debug, P> fmt::Debug for ChunkByMut<'a, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkByMut").field("slice", &self.slice).finish()
    }
}
```

---
*Generated by AST tracing system*
