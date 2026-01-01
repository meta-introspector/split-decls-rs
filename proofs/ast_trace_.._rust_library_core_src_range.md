# AST Trace: ../rust/library/core/src/range.rs

Generated 43 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
//! # Experimental replacement range types
//!
//! The types within this module are meant to replace the existing
//! `Range`, `RangeInclusive`, and `RangeFrom` types in a future edition.
//!
//! ```
//! #![feature(new_range_api)]
//! use core::range::{Range, RangeFrom, RangeInclusive};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=19

```rust
//!
//! let arr = [0, 1, 2, 3, 4];
//! assert_eq!(arr[                      ..   ], [0, 1, 2, 3, 4]);
//! assert_eq!(arr[                      .. 3 ], [0, 1, 2      ]);
//! assert_eq!(arr[                      ..=3 ], [0, 1, 2, 3   ]);
//! assert_eq!(arr[     RangeFrom::from(1..  )], [   1, 2, 3, 4]);
//! assert_eq!(arr[         Range::from(1..3 )], [   1, 2      ]);
//! assert_eq!(arr[RangeInclusive::from(1..=3)], [   1, 2, 3   ]);
//! ```

use crate::fmt;
use crate::hash::Hash;

mod iter;

#[unstable(feature = "new_range_api", issue = "125687")]
pub mod legacy;

use Bound::{Excluded, Included, Unbounded};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[doc(inline)]
pub use iter::{IterRange, IterRangeFrom, IterRangeInclusive};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[doc(inline)]
pub use crate::iter::Step;
#[doc(inline)]
pub use crate::ops::{Bound, IntoBounds, OneSidedRange, RangeBounds, RangeFull, RangeTo};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
/// A (half-open) range bounded inclusively below and exclusively above
/// (`start..end` in a future edition).
///
/// The range `start..end` contains all values with `start <= x < end`.
/// It is empty if `start >= end`.
///
/// # Examples
///
/// ```
/// #![feature(new_range_api)]
/// use core::range::Range;
///
/// assert_eq!(Range::from(3..5), Range { start: 3, end: 5 });
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=Range | COMPLEXITY=3 | LINES=14

```rust
/// assert_eq!(3 + 4 + 5, Range::from(3..6).into_iter().sum());
/// ```
#[lang = "RangeCopy"]
#[derive(Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq)]
#[unstable(feature = "new_range_api", issue = "125687")]
pub struct Range<Idx> {
    /// The lower bound of the range (inclusive).
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub start: Idx,
    /// The upper bound of the range (exclusive).
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub end: Idx,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=iter | COMPLEXITY=6 | LINES=23

```rust
impl<Idx: Step> Range<Idx> {
    /// Creates an iterator over the elements within this range.
    ///
    /// Shorthand for `.clone().into_iter()`
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::Range;
    ///
    /// let mut i = Range::from(3..9).iter().map(|n| n*n);
    /// assert_eq!(i.next(), Some(9));
    /// assert_eq!(i.next(), Some(16));
    /// assert_eq!(i.next(), Some(25));
    /// ```
    #[unstable(feature = "new_range_api", issue = "125687")]
    #[inline]
    pub fn iter(&self) -> IterRange<Idx> {
        self.clone().into_iter()
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=13 | LINES=62

```rust
impl<Idx: PartialOrd<Idx>> Range<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::Range;
    ///
    /// assert!(!Range::from(3..5).contains(&2));
    /// assert!( Range::from(3..5).contains(&3));
    /// assert!( Range::from(3..5).contains(&4));
    /// assert!(!Range::from(3..5).contains(&5));
    ///
    /// assert!(!Range::from(3..3).contains(&3));
    /// assert!(!Range::from(3..2).contains(&3));
    ///
    /// assert!( Range::from(0.0..1.0).contains(&0.5));
    /// assert!(!Range::from(0.0..1.0).contains(&f32::NAN));
    /// assert!(!Range::from(0.0..f32::NAN).contains(&0.5));
    /// assert!(!Range::from(f32::NAN..1.0).contains(&0.5));
    /// ```
    #[inline]
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::Range;
    ///
    /// assert!(!Range::from(3..5).is_empty());
    /// assert!( Range::from(3..3).is_empty());
    /// assert!( Range::from(3..2).is_empty());
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::Range;
    ///
    /// assert!(!Range::from(3.0..5.0).is_empty());
    /// assert!( Range::from(3.0..f32::NAN).is_empty());
    /// assert!( Range::from(f32::NAN..5.0).is_empty());
    /// ```
    #[inline]
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..end` with `(Bound::Included(start), Bound::Excluded(end))`.
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for Range<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=8

```rust
// #[unstable(feature = "range_into_bounds", issue = "136903")]
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> IntoBounds<T> for Range<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Included(self.start), Excluded(self.end))
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_convert", issue = "143773")]
impl<T> const From<Range<T>> for legacy::Range<T> {
    #[inline]
    fn from(value: Range<T>) -> Self {
        Self { start: value.start, end: value.end }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_convert", issue = "143773")]
impl<T> const From<legacy::Range<T>> for Range<T> {
    #[inline]
    fn from(value: legacy::Range<T>) -> Self {
        Self { start: value.start, end: value.end }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
/// A range bounded inclusively below and above (`start..=last`).
///
/// The `RangeInclusive` `start..=last` contains all values with `x >= start`
/// and `x <= last`. It is empty unless `start <= last`.
///
/// # Examples
///
/// The `start..=last` syntax is a `RangeInclusive`:
///
/// ```
/// #![feature(new_range_api)]
/// use core::range::RangeInclusive;
///
/// assert_eq!(RangeInclusive::from(3..=5), RangeInclusive { start: 3, last: 5 });
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=RangeInclusive | COMPLEXITY=3 | LINES=13

```rust
/// assert_eq!(3 + 4 + 5, RangeInclusive::from(3..=5).into_iter().sum());
/// ```
#[lang = "RangeInclusiveCopy"]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[unstable(feature = "new_range_api", issue = "125687")]
pub struct RangeInclusive<Idx> {
    /// The lower bound of the range (inclusive).
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub start: Idx,
    /// The upper bound of the range (inclusive).
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub last: Idx,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<Idx: fmt::Debug> fmt::Debug for RangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..=")?;
        self.last.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=14 | LINES=63

```rust
impl<Idx: PartialOrd<Idx>> RangeInclusive<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeInclusive;
    ///
    /// assert!(!RangeInclusive::from(3..=5).contains(&2));
    /// assert!( RangeInclusive::from(3..=5).contains(&3));
    /// assert!( RangeInclusive::from(3..=5).contains(&4));
    /// assert!( RangeInclusive::from(3..=5).contains(&5));
    /// assert!(!RangeInclusive::from(3..=5).contains(&6));
    ///
    /// assert!( RangeInclusive::from(3..=3).contains(&3));
    /// assert!(!RangeInclusive::from(3..=2).contains(&3));
    ///
    /// assert!( RangeInclusive::from(0.0..=1.0).contains(&1.0));
    /// assert!(!RangeInclusive::from(0.0..=1.0).contains(&f32::NAN));
    /// assert!(!RangeInclusive::from(0.0..=f32::NAN).contains(&0.0));
    /// assert!(!RangeInclusive::from(f32::NAN..=1.0).contains(&1.0));
    /// ```
    #[inline]
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeInclusive;
    ///
    /// assert!(!RangeInclusive::from(3..=5).is_empty());
    /// assert!(!RangeInclusive::from(3..=3).is_empty());
    /// assert!( RangeInclusive::from(3..=2).is_empty());
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeInclusive;
    ///
    /// assert!(!RangeInclusive::from(3.0..=5.0).is_empty());
    /// assert!( RangeInclusive::from(3.0..=f32::NAN).is_empty());
    /// assert!( RangeInclusive::from(f32::NAN..=5.0).is_empty());
    /// ```
    #[unstable(feature = "new_range_api", issue = "125687")]
    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.start <= self.last)
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=iter | COMPLEXITY=6 | LINES=23

```rust
impl<Idx: Step> RangeInclusive<Idx> {
    /// Creates an iterator over the elements within this range.
    ///
    /// Shorthand for `.clone().into_iter()`
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeInclusive;
    ///
    /// let mut i = RangeInclusive::from(3..=8).iter().map(|n| n*n);
    /// assert_eq!(i.next(), Some(9));
    /// assert_eq!(i.next(), Some(16));
    /// assert_eq!(i.next(), Some(25));
    /// ```
    #[unstable(feature = "new_range_api", issue = "125687")]
    #[inline]
    pub fn iter(&self) -> IterRangeInclusive<Idx> {
        self.clone().into_iter()
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=9

```rust
impl RangeInclusive<usize> {
    /// Converts to an exclusive `Range` for `SliceIndex` implementations.
    /// The caller is responsible for dealing with `last == usize::MAX`.
    #[inline]
    pub(crate) const fn into_slice_range(self) -> Range<usize> {
        Range { start: self.start, end: self.last + 1 }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.last)
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..=end` with `(Bound::Included(start), Bound::Included(end))`.
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for RangeInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(self.last)
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=8

```rust
// #[unstable(feature = "range_into_bounds", issue = "136903")]
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> IntoBounds<T> for RangeInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Included(self.start), Included(self.last))
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_convert", issue = "143773")]
impl<T> const From<RangeInclusive<T>> for legacy::RangeInclusive<T> {
    #[inline]
    fn from(value: RangeInclusive<T>) -> Self {
        Self::new(value.start, value.last)
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=from | COMPLEXITY=7 | LINES=14

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_convert", issue = "143773")]
impl<T> const From<legacy::RangeInclusive<T>> for RangeInclusive<T> {
    #[inline]
    fn from(value: legacy::RangeInclusive<T>) -> Self {
        assert!(
            !value.exhausted,
            "attempted to convert from an exhausted `legacy::RangeInclusive` (unspecified behavior)"
        );

        let (start, last) = value.into_inner();
        RangeInclusive { start, last }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=25

```rust
/// A range only bounded inclusively below (`start..`).
///
/// The `RangeFrom` `start..` contains all values with `x >= start`.
///
/// *Note*: Overflow in the [`Iterator`] implementation (when the contained
/// data type reaches its numerical limit) is allowed to panic, wrap, or
/// saturate. This behavior is defined by the implementation of the [`Step`]
/// trait. For primitive integers, this follows the normal rules, and respects
/// the overflow checks profile (panic in debug, wrap in release). Note also
/// that overflow happens earlier than you might assume: the overflow happens
/// in the call to `next` that yields the maximum value, as the range must be
/// set to a state to yield the next value.
///
/// [`Step`]: crate::iter::Step
///
/// # Examples
///
/// The `start..` syntax is a `RangeFrom`:
///
/// ```
/// #![feature(new_range_api)]
/// use core::range::RangeFrom;
///
/// assert_eq!(RangeFrom::from(2..), core::range::RangeFrom { start: 2 });
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=STRUCT | NAME=RangeFrom | COMPLEXITY=2 | LINES=11

```rust
/// assert_eq!(2 + 3 + 4, RangeFrom::from(2..).into_iter().take(3).sum());
/// ```
#[lang = "RangeFromCopy"]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[unstable(feature = "new_range_api", issue = "125687")]
pub struct RangeFrom<Idx> {
    /// The lower bound of the range (inclusive).
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub start: Idx,
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<Idx: fmt::Debug> fmt::Debug for RangeFrom<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        Ok(())
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=iter | COMPLEXITY=6 | LINES=23

```rust
impl<Idx: Step> RangeFrom<Idx> {
    /// Creates an iterator over the elements within this range.
    ///
    /// Shorthand for `.clone().into_iter()`
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeFrom;
    ///
    /// let mut i = RangeFrom::from(3..).iter().map(|n| n*n);
    /// assert_eq!(i.next(), Some(9));
    /// assert_eq!(i.next(), Some(16));
    /// assert_eq!(i.next(), Some(25));
    /// ```
    #[unstable(feature = "new_range_api", issue = "125687")]
    #[inline]
    pub fn iter(&self) -> IterRangeFrom<Idx> {
        self.clone().into_iter()
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=6 | LINES=28

```rust
impl<Idx: PartialOrd<Idx>> RangeFrom<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// use core::range::RangeFrom;
    ///
    /// assert!(!RangeFrom::from(3..).contains(&2));
    /// assert!( RangeFrom::from(3..).contains(&3));
    /// assert!( RangeFrom::from(3..).contains(&1_000_000_000));
    ///
    /// assert!( RangeFrom::from(0.0..).contains(&0.5));
    /// assert!(!RangeFrom::from(0.0..).contains(&f32::NAN));
    /// assert!(!RangeFrom::from(f32::NAN..).contains(&0.5));
    /// ```
    #[inline]
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..` with `(Bound::Included(start), Bound::Unbounded)`.
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for RangeFrom<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=8

```rust
// #[unstable(feature = "range_into_bounds", issue = "136903")]
#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> IntoBounds<T> for RangeFrom<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Included(self.start), Unbounded)
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_index", issue = "143775")]
impl<T> const From<RangeFrom<T>> for legacy::RangeFrom<T> {
    #[inline]
    fn from(value: RangeFrom<T>) -> Self {
        Self { start: value.start }
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=8

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
#[rustc_const_unstable(feature = "const_index", issue = "143775")]
impl<T> const From<legacy::RangeFrom<T>> for RangeFrom<T> {
    #[inline]
    fn from(value: legacy::RangeFrom<T>) -> Self {
        Self { start: value.start }
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
/// A range only bounded inclusively above (`..=last`).
///
/// The `RangeToInclusive` `..=last` contains all values with `x <= last`.
/// It cannot serve as an [`Iterator`] because it doesn't have a starting point.
///
/// # Examples
///
/// The `..=last` syntax is a `RangeToInclusive`:
///
/// ```
/// #![feature(new_range_api)]
/// #![feature(new_range)]
/// assert_eq!((..=5), std::range::RangeToInclusive{ last: 5 });
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
/// ```
///
/// It does not have an [`IntoIterator`] implementation, so you can't use it in a
/// `for` loop directly. This won't compile:
///
/// ```compile_fail,E0277
/// // error[E0277]: the trait bound `std::range::RangeToInclusive<{integer}>:
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
/// // std::iter::Iterator` is not satisfied
/// for i in ..=5 {
///     // ...
/// }
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=STRUCT | NAME=RangeToInclusive | COMPLEXITY=3 | LINES=25

```rust
/// ```
///
/// When used as a [slicing index], `RangeToInclusive` produces a slice of all
/// array elements up to and including the index indicated by `last`.
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]); // This is a `RangeToInclusive`
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
///
/// [slicing index]: crate::slice::SliceIndex
#[lang = "RangeToInclusiveCopy"]
#[doc(alias = "..=")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[unstable(feature = "new_range_api", issue = "125687")]
pub struct RangeToInclusive<Idx> {
    /// The upper bound of the range (inclusive)
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub last: Idx,
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
#[unstable(feature = "new_range_api", issue = "125687")]
impl<Idx: fmt::Debug> fmt::Debug for RangeToInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..=")?;
        self.last.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=6 | LINES=25

```rust
impl<Idx: PartialOrd<Idx>> RangeToInclusive<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!( (..=5).contains(&-1_000_000_000));
    /// assert!( (..=5).contains(&5));
    /// assert!(!(..=5).contains(&6));
    ///
    /// assert!( (..=1.0).contains(&1.0));
    /// assert!(!(..=1.0).contains(&f32::NAN));
    /// assert!(!(..=f32::NAN).contains(&0.5));
    /// ```
    #[inline]
    #[unstable(feature = "new_range_api", issue = "125687")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=13

```rust
// RangeToInclusive<Idx> cannot impl From<RangeTo<Idx>>
// because underflow would be possible with (..0).into()

#[unstable(feature = "new_range_api", issue = "125687")]
impl<T> RangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.last)
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeToInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Unbounded, Included(self.last))
    }
}
```

---
*Generated by AST tracing system*
