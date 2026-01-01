# AST Trace: ../rust/library/core/src/iter/adapters/inspect.rs

Generated 14 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::fmt;
use crate::iter::adapters::SourceIter;
use crate::iter::{FusedIterator, InPlaceIterable, TrustedFused};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=Inspect | COMPLEXITY=5 | LINES=18

```rust
use crate::num::NonZero;
use crate::ops::Try;

/// An iterator that calls a function with a reference to each element before
/// yielding it.
///
/// This `struct` is created by the [`inspect`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`inspect`]: Iterator::inspect
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Inspect<I, F> {
    iter: I,
    f: F,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
impl<I, F> Inspect<I, F> {
    pub(in crate::iter) fn new(iter: I, f: F) -> Inspect<I, F> {
        Inspect { iter, f }
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for Inspect<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inspect").field("iter", &self.iter).finish()
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=do_inspect | COMPLEXITY=6 | LINES=14

```rust
impl<I: Iterator, F> Inspect<I, F>
where
    F: FnMut(&I::Item),
{
    #[inline]
    fn do_inspect(&mut self, elt: Option<I::Item>) -> Option<I::Item> {
        if let Some(ref a) = elt {
            (self.f)(a);
        }

        elt
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=inspect_fold | COMPLEXITY=3 | LINES=10

```rust
fn inspect_fold<T, Acc>(
    mut f: impl FnMut(&T),
    mut fold: impl FnMut(Acc, T) -> Acc,
) -> impl FnMut(Acc, T) -> Acc {
    move |acc, item| {
        f(&item);
        fold(acc, item)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=inspect_try_fold | COMPLEXITY=3 | LINES=10

```rust
fn inspect_try_fold<'a, T, Acc, R>(
    f: &'a mut impl FnMut(&T),
    mut fold: impl FnMut(Acc, T) -> R + 'a,
) -> impl FnMut(Acc, T) -> R + 'a {
    move |acc, item| {
        f(&item);
        fold(acc, item)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=next | COMPLEXITY=9 | LINES=37

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, F> Iterator for Inspect<I, F>
where
    F: FnMut(&I::Item),
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let next = self.iter.next();
        self.do_inspect(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
    where
        Self: Sized,
        Fold: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.iter.try_fold(init, inspect_try_fold(&mut self.f, fold))
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, inspect_fold(self.f, fold))
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=8 | LINES=30

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, F> DoubleEndedIterator for Inspect<I, F>
where
    F: FnMut(&I::Item),
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        let next = self.iter.next_back();
        self.do_inspect(next)
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
    where
        Self: Sized,
        Fold: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.iter.try_rfold(init, inspect_try_fold(&mut self.f, fold))
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, inspect_fold(self.f, fold))
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=len | COMPLEXITY=6 | LINES=14

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator, F> ExactSizeIterator for Inspect<I, F>
where
    F: FnMut(&I::Item),
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<I: FusedIterator, F> FusedIterator for Inspect<I, F> where F: FnMut(&I::Item) {}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(issue = "none", feature = "trusted_fused")]
unsafe impl<I: TrustedFused, F> TrustedFused for Inspect<I, F> {}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=26 | LINES=14

```rust
#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I, F> SourceIter for Inspect<I, F>
where
    I: SourceIter,
{
    type Source = I::Source;

    #[inline]
    unsafe fn as_inner(&mut self) -> &mut I::Source {
        // SAFETY: unsafe function forwarding to unsafe function with the same requirements
        unsafe { SourceIter::as_inner(&mut self.iter) }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I: InPlaceIterable, F> InPlaceIterable for Inspect<I, F> {
    const EXPAND_BY: Option<NonZero<usize>> = I::EXPAND_BY;
    const MERGE_BY: Option<NonZero<usize>> = I::MERGE_BY;
}
```

---
*Generated by AST tracing system*
