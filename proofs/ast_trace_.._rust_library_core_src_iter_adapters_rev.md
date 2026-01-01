# AST Trace: ../rust/library/core/src/iter/adapters/rev.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::iter::{FusedIterator, TrustedLen};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=Rev | COMPLEXITY=4 | LINES=16

```rust
use crate::num::NonZero;
use crate::ops::Try;

/// A double-ended iterator with the direction inverted.
///
/// This `struct` is created by the [`rev`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`rev`]: Iterator::rev
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Rev<T> {
    iter: T,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=into_inner | COMPLEXITY=6 | LINES=25

```rust
impl<T> Rev<T> {
    pub(in crate::iter) fn new(iter: T) -> Rev<T> {
        Rev { iter }
    }

    /// Consumes the `Rev`, returning the inner iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(rev_into_inner)]
    ///
    /// let s = "foobar";
    /// let mut rev = s.chars().rev();
    /// assert_eq!(rev.next(), Some('r'));
    /// assert_eq!(rev.next(), Some('a'));
    /// assert_eq!(rev.next(), Some('b'));
    /// assert_eq!(rev.into_inner().collect::<String>(), "foo");
    /// ```
    #[unstable(feature = "rev_into_inner", issue = "144277")]
    pub fn into_inner(self) -> T {
        self.iter
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=next | COMPLEXITY=13 | LINES=51

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Rev<I>
where
    I: DoubleEndedIterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.iter.next_back()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        self.iter.advance_back_by(n)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<<I as Iterator>::Item> {
        self.iter.nth_back(n)
    }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        self.iter.try_rfold(init, f)
    }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, f)
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.iter.rfind(predicate)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=11 | LINES=44

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Rev<I>
where
    I: DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
        self.iter.next()
    }

    #[inline]
    fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        self.iter.advance_by(n)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<<I as Iterator>::Item> {
        self.iter.nth(n)
    }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        self.iter.try_fold(init, f)
    }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, f)
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.iter.find(predicate)
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=len | COMPLEXITY=6 | LINES=14

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Rev<I>
where
    I: ExactSizeIterator + DoubleEndedIterator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<I> FusedIterator for Rev<I> where I: FusedIterator + DoubleEndedIterator {}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I> TrustedLen for Rev<I> where I: TrustedLen + DoubleEndedIterator {}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=default | COMPLEXITY=5 | LINES=14

```rust
#[stable(feature = "default_iters", since = "1.70.0")]
impl<I: Default> Default for Rev<I> {
    /// Creates a `Rev` iterator from the default value of `I`
    /// ```
    /// # use core::slice;
    /// # use core::iter::Rev;
    /// let iter: Rev<slice::Iter<'_, u8>> = Default::default();
    /// assert_eq!(iter.len(), 0);
    /// ```
    fn default() -> Self {
        Rev::new(Default::default())
    }
}
```

---
*Generated by AST tracing system*
