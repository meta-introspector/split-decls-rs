# AST Trace: ../rust/compiler/rustc_index/src/idx.rs

Generated 15 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=24

```rust
use std::fmt::Debug;
use std::hash::Hash;
use std::ops;
use std::slice::SliceIndex;

/// Represents some newtyped `usize` wrapper.
///
/// Purpose: avoid mixing indexes for different bitvector domains.
pub trait Idx: Copy + 'static + Eq + PartialEq + Debug + Hash {
    fn new(idx: usize) -> Self;

    fn index(self) -> usize;

    #[inline]
    fn increment_by(&mut self, amount: usize) {
        *self = self.plus(amount);
    }

    #[inline]
    #[must_use = "Use `increment_by` if you wanted to update the index in-place"]
    fn plus(self, amount: usize) -> Self {
        Self::new(self.index() + amount)
    }
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=11

```rust
impl Idx for usize {
    #[inline]
    fn new(idx: usize) -> Self {
        idx
    }
    #[inline]
    fn index(self) -> usize {
        self
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=12

```rust
impl Idx for u32 {
    #[inline]
    fn new(idx: usize) -> Self {
        assert!(idx <= u32::MAX as usize);
        idx as u32
    }
    #[inline]
    fn index(self) -> usize {
        self as usize
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=4 | LINES=6

```rust
/// Helper trait for indexing operations with a custom index type.
pub trait IntoSliceIdx<I, T: ?Sized> {
    type Output: SliceIndex<T>;
    fn into_slice_idx(self) -> Self::Output;
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=5 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for I {
    type Output = usize;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        self.index()
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=5 | LINES=8

```rust
impl<I, T> IntoSliceIdx<I, [T]> for ops::RangeFull {
    type Output = ops::RangeFull;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        self
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=6 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for ops::Range<I> {
    type Output = ops::Range<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        ops::Range { start: self.start.index(), end: self.end.index() }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=6 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for ops::RangeFrom<I> {
    type Output = ops::RangeFrom<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        ops::RangeFrom { start: self.start.index() }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=5 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for ops::RangeTo<I> {
    type Output = ops::RangeTo<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        ..self.end.index()
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=5 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for ops::RangeInclusive<I> {
    type Output = ops::RangeInclusive<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        ops::RangeInclusive::new(self.start().index(), self.end().index())
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=5 | LINES=8

```rust
impl<I: Idx, T> IntoSliceIdx<I, [T]> for ops::RangeToInclusive<I> {
    type Output = ops::RangeToInclusive<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        ..=self.end.index()
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=6 | LINES=9

```rust
#[cfg(feature = "nightly")]
impl<I: Idx, T> IntoSliceIdx<I, [T]> for core::range::Range<I> {
    type Output = core::range::Range<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        core::range::Range { start: self.start.index(), end: self.end.index() }
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=6 | LINES=9

```rust
#[cfg(feature = "nightly")]
impl<I: Idx, T> IntoSliceIdx<I, [T]> for core::range::RangeFrom<I> {
    type Output = core::range::RangeFrom<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        core::range::RangeFrom { start: self.start.index() }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=9 | LINES=15

```rust
#[cfg(feature = "nightly")]
impl<I: Idx, T> IntoSliceIdx<I, [T]> for core::range::RangeInclusive<I> {
    type Output = core::range::RangeInclusive<usize>;
    #[inline]
    #[cfg(bootstrap)]
    fn into_slice_idx(self) -> Self::Output {
        core::range::RangeInclusive { start: self.start.index(), end: self.end.index() }
    }
    #[inline]
    #[cfg(not(bootstrap))]
    fn into_slice_idx(self) -> Self::Output {
        core::range::RangeInclusive { start: self.start.index(), last: self.last.index() }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=into_slice_idx | COMPLEXITY=6 | LINES=9

```rust
#[cfg(all(feature = "nightly", not(bootstrap)))]
impl<I: Idx, T> IntoSliceIdx<I, [T]> for core::range::RangeToInclusive<I> {
    type Output = core::range::RangeToInclusive<usize>;
    #[inline]
    fn into_slice_idx(self) -> Self::Output {
        core::range::RangeToInclusive { last: self.last.index() }
    }
}
```

---
*Generated by AST tracing system*
