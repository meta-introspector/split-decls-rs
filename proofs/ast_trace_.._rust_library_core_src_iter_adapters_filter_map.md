# AST Trace: ../rust/library/core/src/iter/adapters/filter_map.rs

Generated 15 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::iter::adapters::SourceIter;
use crate::iter::{FusedIterator, InPlaceIterable, TrustedFused};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::mem::{ManuallyDrop, MaybeUninit};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::num::NonZero;
use crate::ops::{ControlFlow, Try};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{array, fmt};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=FilterMap | COMPLEXITY=4 | LINES=15

```rust
/// An iterator that uses `f` to both filter and map elements from `iter`.
///
/// This `struct` is created by the [`filter_map`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`filter_map`]: Iterator::filter_map
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
impl<I, F> FilterMap<I, F> {
    pub(in crate::iter) fn new(iter: I, f: F) -> FilterMap<I, F> {
        FilterMap { iter, f }
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for FilterMap<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterMap").field("iter", &self.iter).finish()
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=filter_map_fold | COMPLEXITY=6 | LINES=10

```rust
fn filter_map_fold<T, B, Acc>(
    mut f: impl FnMut(T) -> Option<B>,
    mut fold: impl FnMut(Acc, B) -> Acc,
) -> impl FnMut(Acc, T) -> Acc {
    move |acc, item| match f(item) {
        Some(x) => fold(acc, x),
        None => acc,
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=filter_map_try_fold | COMPLEXITY=7 | LINES=10

```rust
fn filter_map_try_fold<'a, T, B, Acc, R: Try<Output = Acc>>(
    f: &'a mut impl FnMut(T) -> Option<B>,
    mut fold: impl FnMut(Acc, B) -> R + 'a,
) -> impl FnMut(Acc, T) -> R + 'a {
    move |acc, item| match f(item) {
        Some(x) => fold(acc, x),
        None => try { acc },
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=next | COMPLEXITY=61 | LINES=95

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.find_map(&mut self.f)
    }

    #[inline]
    fn next_chunk<const N: usize>(
        &mut self,
    ) -> Result<[Self::Item; N], array::IntoIter<Self::Item, N>> {
        let mut array: [MaybeUninit<Self::Item>; N] = [const { MaybeUninit::uninit() }; N];

        struct Guard<'a, T> {
            array: &'a mut [MaybeUninit<T>],
            initialized: usize,
        }

        impl<T> Drop for Guard<'_, T> {
            #[inline]
            fn drop(&mut self) {
                if const { crate::mem::needs_drop::<T>() } {
                    // SAFETY: self.initialized is always <= N, which also is the length of the array.
                    unsafe {
                        self.array.get_unchecked_mut(..self.initialized).assume_init_drop();
                    }
                }
            }
        }

        let mut guard = Guard { array: &mut array, initialized: 0 };

        let result = self.iter.try_for_each(|element| {
            let idx = guard.initialized;
            let val = (self.f)(element);
            guard.initialized = idx + val.is_some() as usize;

            // SAFETY: Loop conditions ensure the index is in bounds.

            unsafe {
                let opt_payload_at: *const MaybeUninit<B> =
                    (&raw const val).byte_add(core::mem::offset_of!(Option<B>, Some.0)).cast();
                let dst = guard.array.as_mut_ptr().add(idx);
                crate::ptr::copy_nonoverlapping(opt_payload_at, dst, 1);
                crate::mem::forget(val);
            };

            if guard.initialized < N { ControlFlow::Continue(()) } else { ControlFlow::Break(()) }
        });

        let guard = ManuallyDrop::new(guard);

        match result {
            ControlFlow::Break(()) => {
                // SAFETY: The loop above is only explicitly broken when the array has been fully initialized
                Ok(unsafe { MaybeUninit::array_assume_init(array) })
            }
            ControlFlow::Continue(()) => {
                let initialized = guard.initialized;
                // SAFETY: The range is in bounds since the loop breaks when reaching N elements.
                Err(unsafe { array::IntoIter::new_unchecked(array, 0..initialized) })
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
    where
        Self: Sized,
        Fold: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.iter.try_fold(init, filter_map_try_fold(&mut self.f, fold))
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, filter_map_fold(self.f, fold))
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=14 | LINES=39

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for FilterMap<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        #[inline]
        fn find<T, B>(
            f: &mut impl FnMut(T) -> Option<B>,
        ) -> impl FnMut((), T) -> ControlFlow<B> + '_ {
            move |(), x| match f(x) {
                Some(x) => ControlFlow::Break(x),
                None => ControlFlow::Continue(()),
            }
        }

        self.iter.try_rfold((), find(&mut self.f)).break_value()
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
    where
        Self: Sized,
        Fold: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.iter.try_rfold(init, filter_map_try_fold(&mut self.f, fold))
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, filter_map_fold(self.f, fold))
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F> where F: FnMut(I::Item) -> Option<B> {}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
#[unstable(issue = "none", feature = "trusted_fused")]
unsafe impl<I: TrustedFused, F> TrustedFused for FilterMap<I, F> {}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=26 | LINES=14

```rust
#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I, F> SourceIter for FilterMap<I, F>
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

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I: InPlaceIterable, F> InPlaceIterable for FilterMap<I, F> {
    const EXPAND_BY: Option<NonZero<usize>> = I::EXPAND_BY;
    const MERGE_BY: Option<NonZero<usize>> = I::MERGE_BY;
}
```

---
*Generated by AST tracing system*
