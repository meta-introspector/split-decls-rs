# AST Trace: ../rust/library/core/src/ops/range.rs

Generated 61 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=7 | LINES=23

```rust
use crate::fmt;
use crate::hash::Hash;

/// An unbounded range (`..`).
///
/// `RangeFull` is primarily used as a [slicing index], its shorthand is `..`.
/// It cannot serve as an [`Iterator`] because it doesn't have a starting point.
///
/// # Examples
///
/// The `..` syntax is a `RangeFull`:
///
/// ```
/// assert_eq!(.., std::ops::RangeFull);
/// ```
///
/// It does not have an [`IntoIterator`] implementation, so you can't use it in
/// a `for` loop directly. This won't compile:
///
/// ```compile_fail,E0277
/// for i in .. {
///     // ...
/// }
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=RangeFull; | COMPLEXITY=6 | LINES=27

```rust
/// ```
///
/// Used as a [slicing index], `RangeFull` produces the full array as a slice.
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]); // This is the `RangeFull`
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
///
/// [slicing index]: crate::slice::SliceIndex
#[lang = "RangeFull"]
#[doc(alias = "..")]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeFull;

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for RangeFull {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..")
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
/// A (half-open) range bounded inclusively below and exclusively above
/// (`start..end`).
///
/// The range `start..end` contains all values with `start <= x < end`.
/// It is empty if `start >= end`.
///
/// # Examples
///
/// The `start..end` syntax is a `Range`:
///
/// ```
/// assert_eq!((3..5), std::ops::Range { start: 3, end: 5 });
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=Range | COMPLEXITY=3 | LINES=24

```rust
/// assert_eq!(3 + 4 + 5, (3..6).sum());
/// ```
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]); // This is a `Range`
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
#[lang = "Range"]
#[doc(alias = "..")]
#[derive(Clone, Default, PartialEq, Eq, Hash)] // not Copy -- see #27186
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Range<Idx> {
    /// The lower bound of the range (inclusive).
    #[stable(feature = "rust1", since = "1.0.0")]
    pub start: Idx,
    /// The upper bound of the range (exclusive).
    #[stable(feature = "rust1", since = "1.0.0")]
    pub end: Idx,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=12 | LINES=53

```rust
impl<Idx: PartialOrd<Idx>> Range<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(!(3..5).contains(&2));
    /// assert!( (3..5).contains(&3));
    /// assert!( (3..5).contains(&4));
    /// assert!(!(3..5).contains(&5));
    ///
    /// assert!(!(3..3).contains(&3));
    /// assert!(!(3..2).contains(&3));
    ///
    /// assert!( (0.0..1.0).contains(&0.5));
    /// assert!(!(0.0..1.0).contains(&f32::NAN));
    /// assert!(!(0.0..f32::NAN).contains(&0.5));
    /// assert!(!(f32::NAN..1.0).contains(&0.5));
    /// ```
    #[inline]
    #[stable(feature = "range_contains", since = "1.35.0")]
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
    /// assert!(!(3..5).is_empty());
    /// assert!( (3..3).is_empty());
    /// assert!( (3..2).is_empty());
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// assert!(!(3.0..5.0).is_empty());
    /// assert!( (3.0..f32::NAN).is_empty());
    /// assert!( (f32::NAN..5.0).is_empty());
    /// ```
    #[inline]
    #[stable(feature = "range_is_empty", since = "1.47.0")]
    pub fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=22

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
/// assert_eq!((2..), std::ops::RangeFrom { start: 2 });
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=RangeFrom | COMPLEXITY=3 | LINES=21

```rust
/// assert_eq!(2 + 3 + 4, (2..).take(3).sum());
/// ```
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]); // This is a `RangeFrom`
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
#[lang = "RangeFrom"]
#[doc(alias = "..")]
#[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeFrom<Idx> {
    /// The lower bound of the range (inclusive).
    #[stable(feature = "rust1", since = "1.0.0")]
    pub start: Idx,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeFrom<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        Ok(())
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=6 | LINES=25

```rust
impl<Idx: PartialOrd<Idx>> RangeFrom<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(!(3..).contains(&2));
    /// assert!( (3..).contains(&3));
    /// assert!( (3..).contains(&1_000_000_000));
    ///
    /// assert!( (0.0..).contains(&0.5));
    /// assert!(!(0.0..).contains(&f32::NAN));
    /// assert!(!(f32::NAN..).contains(&0.5));
    /// ```
    #[inline]
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
/// A range only bounded exclusively above (`..end`).
///
/// The `RangeTo` `..end` contains all values with `x < end`.
/// It cannot serve as an [`Iterator`] because it doesn't have a starting point.
///
/// # Examples
///
/// The `..end` syntax is a `RangeTo`:
///
/// ```
/// assert_eq!((..5), std::ops::RangeTo { end: 5 });
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
/// ```
///
/// It does not have an [`IntoIterator`] implementation, so you can't use it in
/// a `for` loop directly. This won't compile:
///
/// ```compile_fail,E0277
/// // error[E0277]: the trait bound `std::ops::RangeTo<{integer}>:
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
/// // std::iter::Iterator` is not satisfied
/// for i in ..5 {
///     // ...
/// }
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=RangeTo | COMPLEXITY=3 | LINES=25

```rust
/// ```
///
/// When used as a [slicing index], `RangeTo` produces a slice of all array
/// elements before the index indicated by `end`.
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]); // This is a `RangeTo`
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
///
/// [slicing index]: crate::slice::SliceIndex
#[lang = "RangeTo"]
#[doc(alias = "..")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RangeTo<Idx> {
    /// The upper bound of the range (exclusive).
    #[stable(feature = "rust1", since = "1.0.0")]
    pub end: Idx,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeTo<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=6 | LINES=25

```rust
impl<Idx: PartialOrd<Idx>> RangeTo<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!( (..5).contains(&-1_000_000_000));
    /// assert!( (..5).contains(&4));
    /// assert!(!(..5).contains(&5));
    ///
    /// assert!( (..1.0).contains(&0.5));
    /// assert!(!(..1.0).contains(&f32::NAN));
    /// assert!(!(..f32::NAN).contains(&0.5));
    /// ```
    #[inline]
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=RangeInclusive | COMPLEXITY=8 | LINES=52

```rust
/// A range bounded inclusively below and above (`start..=end`).
///
/// The `RangeInclusive` `start..=end` contains all values with `x >= start`
/// and `x <= end`. It is empty unless `start <= end`.
///
/// This iterator is [fused], but the specific values of `start` and `end` after
/// iteration has finished are **unspecified** other than that [`.is_empty()`]
/// will return `true` once no more values will be produced.
///
/// [fused]: crate::iter::FusedIterator
/// [`.is_empty()`]: RangeInclusive::is_empty
///
/// # Examples
///
/// The `start..=end` syntax is a `RangeInclusive`:
///
/// ```
/// assert_eq!((3..=5), std::ops::RangeInclusive::new(3, 5));
/// assert_eq!(3 + 4 + 5, (3..=5).sum());
/// ```
///
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]); // This is a `RangeInclusive`
/// ```
#[lang = "RangeInclusive"]
#[doc(alias = "..=")]
#[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
#[stable(feature = "inclusive_range", since = "1.26.0")]
pub struct RangeInclusive<Idx> {
    // Note that the fields here are not public to allow changing the
    // representation in the future; in particular, while we could plausibly
    // expose start/end, modifying them without changing (future/current)
    // private fields may lead to incorrect behavior, so we don't want to
    // support that mode.
    pub(crate) start: Idx,
    pub(crate) end: Idx,

    // This field is:
    //  - `false` upon construction
    //  - `false` when iteration has yielded an element and the iterator is not exhausted
    //  - `true` when iteration has been used to exhaust the iterator
    //
    // This is required to support PartialEq and Hash without a PartialOrd bound or specialization.
    pub(crate) exhausted: bool,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=16 | LINES=87

```rust
impl<Idx> RangeInclusive<Idx> {
    /// Creates a new inclusive range. Equivalent to writing `start..=end`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::RangeInclusive;
    ///
    /// assert_eq!(3..=5, RangeInclusive::new(3, 5));
    /// ```
    #[lang = "range_inclusive_new"]
    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    #[rustc_promotable]
    #[rustc_const_stable(feature = "const_range_new", since = "1.32.0")]
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self { start, end, exhausted: false }
    }

    /// Returns the lower bound of the range (inclusive).
    ///
    /// When using an inclusive range for iteration, the values of `start()` and
    /// [`end()`] are unspecified after the iteration ended. To determine
    /// whether the inclusive range is empty, use the [`is_empty()`] method
    /// instead of comparing `start() > end()`.
    ///
    /// Note: the value returned by this method is unspecified after the range
    /// has been iterated to exhaustion.
    ///
    /// [`end()`]: RangeInclusive::end
    /// [`is_empty()`]: RangeInclusive::is_empty
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((3..=5).start(), &3);
    /// ```
    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[rustc_const_stable(feature = "const_inclusive_range_methods", since = "1.32.0")]
    #[inline]
    pub const fn start(&self) -> &Idx {
        &self.start
    }

    /// Returns the upper bound of the range (inclusive).
    ///
    /// When using an inclusive range for iteration, the values of [`start()`]
    /// and `end()` are unspecified after the iteration ended. To determine
    /// whether the inclusive range is empty, use the [`is_empty()`] method
    /// instead of comparing `start() > end()`.
    ///
    /// Note: the value returned by this method is unspecified after the range
    /// has been iterated to exhaustion.
    ///
    /// [`start()`]: RangeInclusive::start
    /// [`is_empty()`]: RangeInclusive::is_empty
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((3..=5).end(), &5);
    /// ```
    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[rustc_const_stable(feature = "const_inclusive_range_methods", since = "1.32.0")]
    #[inline]
    pub const fn end(&self) -> &Idx {
        &self.end
    }

    /// Destructures the `RangeInclusive` into (lower bound, upper (inclusive) bound).
    ///
    /// Note: the value returned by this method is unspecified after the range
    /// has been iterated to exhaustion.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((3..=5).into_inner(), (3, 5));
    /// ```
    #[stable(feature = "inclusive_range_methods", since = "1.27.0")]
    #[inline]
    #[rustc_const_unstable(feature = "const_range_bounds", issue = "108082")]
    pub const fn into_inner(self) -> (Idx, Idx) {
        (self.start, self.end)
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=14

```rust
impl RangeInclusive<usize> {
    /// Converts to an exclusive `Range` for `SliceIndex` implementations.
    /// The caller is responsible for dealing with `end == usize::MAX`.
    #[inline]
    pub(crate) const fn into_slice_range(self) -> Range<usize> {
        // If we're not exhausted, we want to simply slice `start..end + 1`.
        // If we are exhausted, then slicing with `end + 1..end + 1` gives us an
        // empty range that is still subject to bounds-checks for that endpoint.
        let exclusive_end = self.end + 1;
        let start = if self.exhausted { exclusive_end } else { self.start };
        start..exclusive_end
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=8 | LINES=13

```rust
#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        if self.exhausted {
            write!(fmt, " (exhausted)")?;
        }
        Ok(())
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=20 | LINES=73

```rust
impl<Idx: PartialOrd<Idx>> RangeInclusive<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(!(3..=5).contains(&2));
    /// assert!( (3..=5).contains(&3));
    /// assert!( (3..=5).contains(&4));
    /// assert!( (3..=5).contains(&5));
    /// assert!(!(3..=5).contains(&6));
    ///
    /// assert!( (3..=3).contains(&3));
    /// assert!(!(3..=2).contains(&3));
    ///
    /// assert!( (0.0..=1.0).contains(&1.0));
    /// assert!(!(0.0..=1.0).contains(&f32::NAN));
    /// assert!(!(0.0..=f32::NAN).contains(&0.0));
    /// assert!(!(f32::NAN..=1.0).contains(&1.0));
    /// ```
    ///
    /// This method always returns `false` after iteration has finished:
    ///
    /// ```
    /// let mut r = 3..=5;
    /// assert!(r.contains(&3) && r.contains(&5));
    /// for _ in r.by_ref() {}
    /// // Precise field values are unspecified here
    /// assert!(!r.contains(&3) && !r.contains(&5));
    /// ```
    #[inline]
    #[stable(feature = "range_contains", since = "1.35.0")]
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
    /// assert!(!(3..=5).is_empty());
    /// assert!(!(3..=3).is_empty());
    /// assert!( (3..=2).is_empty());
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// assert!(!(3.0..=5.0).is_empty());
    /// assert!( (3.0..=f32::NAN).is_empty());
    /// assert!( (f32::NAN..=5.0).is_empty());
    /// ```
    ///
    /// This method returns `true` after iteration has finished:
    ///
    /// ```
    /// let mut r = 3..=5;
    /// for _ in r.by_ref() {}
    /// // Precise field values are unspecified here
    /// assert!(r.is_empty());
    /// ```
    #[stable(feature = "range_is_empty", since = "1.47.0")]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.exhausted || !(self.start <= self.end)
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
/// A range only bounded inclusively above (`..=end`).
///
/// The `RangeToInclusive` `..=end` contains all values with `x <= end`.
/// It cannot serve as an [`Iterator`] because it doesn't have a starting point.
///
/// # Examples
///
/// The `..=end` syntax is a `RangeToInclusive`:
///
/// ```
/// assert_eq!((..=5), std::ops::RangeToInclusive{ end: 5 });
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
/// ```
///
/// It does not have an [`IntoIterator`] implementation, so you can't use it in a
/// `for` loop directly. This won't compile:
///
/// ```compile_fail,E0277
/// // error[E0277]: the trait bound `std::ops::RangeToInclusive<{integer}>:
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
/// // std::iter::Iterator` is not satisfied
/// for i in ..=5 {
///     // ...
/// }
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=STRUCT | NAME=RangeToInclusive | COMPLEXITY=3 | LINES=25

```rust
/// ```
///
/// When used as a [slicing index], `RangeToInclusive` produces a slice of all
/// array elements up to and including the index indicated by `end`.
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
#[lang = "RangeToInclusive"]
#[doc(alias = "..=")]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "inclusive_range", since = "1.26.0")]
pub struct RangeToInclusive<Idx> {
    /// The upper bound of the range (inclusive)
    #[stable(feature = "inclusive_range", since = "1.26.0")]
    pub end: Idx,
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "inclusive_range", since = "1.26.0")]
impl<Idx: fmt::Debug> fmt::Debug for RangeToInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=6 | LINES=25

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
    #[stable(feature = "range_contains", since = "1.35.0")]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=3 | LINES=25

```rust
// RangeToInclusive<Idx> cannot impl From<RangeTo<Idx>>
// because underflow would be possible with (..0).into()

/// An endpoint of a range of keys.
///
/// # Examples
///
/// `Bound`s are range endpoints:
///
/// ```
/// use std::ops::Bound::*;
/// use std::ops::RangeBounds;
///
/// assert_eq!((..100).start_bound(), Unbounded);
/// assert_eq!((1..12).start_bound(), Included(&1));
/// assert_eq!((1..12).end_bound(), Excluded(&12));
/// ```
///
/// Using a tuple of `Bound`s as an argument to [`BTreeMap::range`].
/// Note that in most cases, it's better to use range syntax (`1..5`) instead.
///
/// ```
/// use std::collections::BTreeMap;
/// use std::ops::Bound::{Excluded, Included, Unbounded};
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
///
/// let mut map = BTreeMap::new();
/// map.insert(3, "a");
/// map.insert(5, "b");
/// map.insert(8, "c");
///
/// for (key, value) in map.range((Excluded(3), Included(8))) {
///     println!("{key}: {value}");
/// }
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=18

```rust
///
/// assert_eq!(Some((&3, &"a")), map.range((Unbounded, Included(5))).next());
/// ```
///
/// [`BTreeMap::range`]: ../../std/collections/btree_map/struct.BTreeMap.html#method.range
#[stable(feature = "collections_bound", since = "1.17.0")]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Bound<T> {
    /// An inclusive bound.
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Included(#[stable(feature = "collections_bound", since = "1.17.0")] T),
    /// An exclusive bound.
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Excluded(#[stable(feature = "collections_bound", since = "1.17.0")] T),
    /// An infinite endpoint. Indicates that there is no bound in this direction.
    #[stable(feature = "collections_bound", since = "1.17.0")]
    Unbounded,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=20 | LINES=55

```rust
impl<T> Bound<T> {
    /// Converts from `&Bound<T>` to `Bound<&T>`.
    #[inline]
    #[stable(feature = "bound_as_ref_shared", since = "1.65.0")]
    pub fn as_ref(&self) -> Bound<&T> {
        match *self {
            Included(ref x) => Included(x),
            Excluded(ref x) => Excluded(x),
            Unbounded => Unbounded,
        }
    }

    /// Converts from `&mut Bound<T>` to `Bound<&mut T>`.
    #[inline]
    #[unstable(feature = "bound_as_ref", issue = "80996")]
    pub fn as_mut(&mut self) -> Bound<&mut T> {
        match *self {
            Included(ref mut x) => Included(x),
            Excluded(ref mut x) => Excluded(x),
            Unbounded => Unbounded,
        }
    }

    /// Maps a `Bound<T>` to a `Bound<U>` by applying a function to the contained value (including
    /// both `Included` and `Excluded`), returning a `Bound` of the same kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Bound::*;
    ///
    /// let bound_string = Included("Hello, World!");
    ///
    /// assert_eq!(bound_string.map(|s| s.len()), Included(13));
    /// ```
    ///
    /// ```
    /// use std::ops::Bound;
    /// use Bound::*;
    ///
    /// let unbounded_string: Bound<String> = Unbounded;
    ///
    /// assert_eq!(unbounded_string.map(|s| s.len()), Unbounded);
    /// ```
    #[inline]
    #[stable(feature = "bound_map", since = "1.77.0")]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Bound<U> {
        match self {
            Unbounded => Unbounded,
            Included(x) => Included(f(x)),
            Excluded(x) => Excluded(f(x)),
        }
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=copied | COMPLEXITY=8 | LINES=25

```rust
impl<T: Copy> Bound<&T> {
    /// Map a `Bound<&T>` to a `Bound<T>` by copying the contents of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(bound_copied)]
    ///
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert_eq!((1..12).start_bound(), Included(&1));
    /// assert_eq!((1..12).start_bound().copied(), Included(1));
    /// ```
    #[unstable(feature = "bound_copied", issue = "145966")]
    #[must_use]
    pub fn copied(self) -> Bound<T> {
        match self {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(*x),
            Bound::Excluded(x) => Bound::Excluded(*x),
        }
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=cloned | COMPLEXITY=10 | LINES=26

```rust
impl<T: Clone> Bound<&T> {
    /// Map a `Bound<&T>` to a `Bound<T>` by cloning the contents of the bound.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// let a1 = String::from("a");
    /// let (a2, a3, a4) = (a1.clone(), a1.clone(), a1.clone());
    ///
    /// assert_eq!(Included(&a1), (a2..).start_bound());
    /// assert_eq!(Included(a3), (a4..).start_bound().cloned());
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    #[stable(feature = "bound_cloned", since = "1.55.0")]
    pub fn cloned(self) -> Bound<T> {
        match self {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(x.clone()),
            Bound::Excluded(x) => Bound::Excluded(x.clone()),
        }
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=34 | LINES=134

```rust
/// `RangeBounds` is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b`, `..=c`, `d..e`, or `f..=g`.
#[stable(feature = "collections_range", since = "1.28.0")]
#[rustc_diagnostic_item = "RangeBounds"]
pub trait RangeBounds<T: ?Sized> {
    /// Start index bound.
    ///
    /// Returns the start value as a `Bound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert_eq!((..10).start_bound(), Unbounded);
    /// assert_eq!((3..10).start_bound(), Included(&3));
    /// ```
    #[stable(feature = "collections_range", since = "1.28.0")]
    fn start_bound(&self) -> Bound<&T>;

    /// End index bound.
    ///
    /// Returns the end value as a `Bound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert_eq!((3..).end_bound(), Unbounded);
    /// assert_eq!((3..10).end_bound(), Excluded(&10));
    /// ```
    #[stable(feature = "collections_range", since = "1.28.0")]
    fn end_bound(&self) -> Bound<&T>;

    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!( (3..5).contains(&4));
    /// assert!(!(3..5).contains(&2));
    ///
    /// assert!( (0.0..1.0).contains(&0.5));
    /// assert!(!(0.0..1.0).contains(&f32::NAN));
    /// assert!(!(0.0..f32::NAN).contains(&0.5));
    /// assert!(!(f32::NAN..1.0).contains(&0.5));
    /// ```
    #[inline]
    #[stable(feature = "range_contains", since = "1.35.0")]
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        (match self.start_bound() {
            Included(start) => start <= item,
            Excluded(start) => start < item,
            Unbounded => true,
        }) && (match self.end_bound() {
            Included(end) => item <= end,
            Excluded(end) => item < end,
            Unbounded => true,
        })
    }

    /// Returns `true` if the range contains no items.
    /// One-sided ranges (`RangeFrom`, etc) always return `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(range_bounds_is_empty)]
    /// use std::ops::RangeBounds;
    ///
    /// assert!(!(3..).is_empty());
    /// assert!(!(..2).is_empty());
    /// assert!(!RangeBounds::is_empty(&(3..5)));
    /// assert!( RangeBounds::is_empty(&(3..3)));
    /// assert!( RangeBounds::is_empty(&(3..2)));
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// #![feature(range_bounds_is_empty)]
    /// use std::ops::RangeBounds;
    ///
    /// assert!(!RangeBounds::is_empty(&(3.0..5.0)));
    /// assert!( RangeBounds::is_empty(&(3.0..f32::NAN)));
    /// assert!( RangeBounds::is_empty(&(f32::NAN..5.0)));
    /// ```
    ///
    /// But never empty if either side is unbounded:
    ///
    /// ```
    /// #![feature(range_bounds_is_empty)]
    /// use std::ops::RangeBounds;
    ///
    /// assert!(!(..0).is_empty());
    /// assert!(!(i32::MAX..).is_empty());
    /// assert!(!RangeBounds::<u8>::is_empty(&(..)));
    /// ```
    ///
    /// `(Excluded(a), Excluded(b))` is only empty if `a >= b`:
    ///
    /// ```
    /// #![feature(range_bounds_is_empty)]
    /// use std::ops::Bound::*;
    /// use std::ops::RangeBounds;
    ///
    /// assert!(!(Excluded(1), Excluded(3)).is_empty());
    /// assert!(!(Excluded(1), Excluded(2)).is_empty());
    /// assert!( (Excluded(1), Excluded(1)).is_empty());
    /// assert!( (Excluded(2), Excluded(1)).is_empty());
    /// assert!( (Excluded(3), Excluded(1)).is_empty());
    /// ```
    #[unstable(feature = "range_bounds_is_empty", issue = "137300")]
    fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        !match (self.start_bound(), self.end_bound()) {
            (Unbounded, _) | (_, Unbounded) => true,
            (Included(start), Excluded(end))
            | (Excluded(start), Included(end))
            | (Excluded(start), Excluded(end)) => start < end,
            (Included(start), Included(end)) => start <= end,
        }
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=30 | LINES=93

```rust
/// Used to convert a range into start and end bounds, consuming the
/// range by value.
///
/// `IntoBounds` is implemented by Rust’s built-in range types, produced
/// by range syntax like `..`, `a..`, `..b`, `..=c`, `d..e`, or `f..=g`.
#[unstable(feature = "range_into_bounds", issue = "136903")]
pub trait IntoBounds<T>: RangeBounds<T> {
    /// Convert this range into the start and end bounds.
    /// Returns `(start_bound, end_bound)`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(range_into_bounds)]
    /// use std::ops::Bound::*;
    /// use std::ops::IntoBounds;
    ///
    /// assert_eq!((0..5).into_bounds(), (Included(0), Excluded(5)));
    /// assert_eq!((..=7).into_bounds(), (Unbounded, Included(7)));
    /// ```
    fn into_bounds(self) -> (Bound<T>, Bound<T>);

    /// Compute the intersection of  `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(range_into_bounds)]
    /// use std::ops::Bound::*;
    /// use std::ops::IntoBounds;
    ///
    /// assert_eq!((3..).intersect(..5), (Included(3), Excluded(5)));
    /// assert_eq!((-12..387).intersect(0..256), (Included(0), Excluded(256)));
    /// assert_eq!((1..5).intersect(..), (Included(1), Excluded(5)));
    /// assert_eq!((1..=9).intersect(0..10), (Included(1), Included(9)));
    /// assert_eq!((7..=13).intersect(8..13), (Included(8), Excluded(13)));
    /// ```
    ///
    /// Combine with `is_empty` to determine if two ranges overlap.
    ///
    /// ```
    /// #![feature(range_into_bounds)]
    /// #![feature(range_bounds_is_empty)]
    /// use std::ops::{RangeBounds, IntoBounds};
    ///
    /// assert!(!(3..).intersect(..5).is_empty());
    /// assert!(!(-12..387).intersect(0..256).is_empty());
    /// assert!((1..5).intersect(6..).is_empty());
    /// ```
    fn intersect<R>(self, other: R) -> (Bound<T>, Bound<T>)
    where
        Self: Sized,
        T: Ord,
        R: Sized + IntoBounds<T>,
    {
        let (self_start, self_end) = IntoBounds::into_bounds(self);
        let (other_start, other_end) = IntoBounds::into_bounds(other);

        let start = match (self_start, other_start) {
            (Included(a), Included(b)) => Included(Ord::max(a, b)),
            (Excluded(a), Excluded(b)) => Excluded(Ord::max(a, b)),
            (Unbounded, Unbounded) => Unbounded,

            (x, Unbounded) | (Unbounded, x) => x,

            (Included(i), Excluded(e)) | (Excluded(e), Included(i)) => {
                if i > e {
                    Included(i)
                } else {
                    Excluded(e)
                }
            }
        };
        let end = match (self_end, other_end) {
            (Included(a), Included(b)) => Included(Ord::min(a, b)),
            (Excluded(a), Excluded(b)) => Excluded(Ord::min(a, b)),
            (Unbounded, Unbounded) => Unbounded,

            (x, Unbounded) | (Unbounded, x) => x,

            (Included(i), Excluded(e)) | (Excluded(e), Included(i)) => {
                if i < e {
                    Included(i)
                } else {
                    Excluded(e)
                }
            }
        };

        (start, end)
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use self::Bound::{Excluded, Included, Unbounded};
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T: ?Sized> RangeBounds<T> for RangeFull {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeFull {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Unbounded, Unbounded)
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeFrom<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Included(self.start), Unbounded)
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeTo<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeTo<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Unbounded, Excluded(self.end))
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for Range<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Included(self.start), Excluded(self.end))
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        if self.exhausted {
            // When the iterator is exhausted, we usually have start == end,
            // but we want the range to appear empty, containing nothing.
            Excluded(&self.end)
        } else {
            Included(&self.end)
        }
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=10 | LINES=16

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (
            Included(self.start),
            if self.exhausted {
                // When the iterator is exhausted, we usually have start == end,
                // but we want the range to appear empty, containing nothing.
                Excluded(self.end)
            } else {
                Included(self.end)
            },
        )
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for RangeToInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Unbounded, Included(self.end))
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=15 | LINES=19

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for (Bound<T>, Bound<T>) {
    fn start_bound(&self) -> Bound<&T> {
        match *self {
            (Included(ref start), _) => Included(start),
            (Excluded(ref start), _) => Excluded(start),
            (Unbounded, _) => Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        match *self {
            (_, Included(ref end)) => Included(end),
            (_, Excluded(ref end)) => Excluded(end),
            (_, Unbounded) => Unbounded,
        }
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=into_bounds | COMPLEXITY=5 | LINES=7

```rust
#[unstable(feature = "range_into_bounds", issue = "136903")]
impl<T> IntoBounds<T> for (Bound<T>, Bound<T>) {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        self
    }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=6 | LINES=11

```rust
#[stable(feature = "collections_range", since = "1.28.0")]
impl<'a, T: ?Sized + 'a> RangeBounds<T> for (Bound<&'a T>, Bound<&'a T>) {
    fn start_bound(&self) -> Bound<&T> {
        self.0
    }

    fn end_bound(&self) -> Bound<&T> {
        self.1
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..` with `(Bound::Included(start), Bound::Unbounded)`.
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeFrom<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `..end` with `(Bound::Unbounded, Bound::Excluded(end))`.
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeTo<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..end` with `(Bound::Included(start), Bound::Excluded(end))`.
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for Range<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(self.end)
    }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `start..=end` with `(Bound::Included(start), Bound::Included(end))`.
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end)
    }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=start_bound | COMPLEXITY=11 | LINES=16

```rust
// This impl intentionally does not have `T: ?Sized`;
// see https://github.com/rust-lang/rust/pull/61584 for discussion of why.
//
/// If you need to use this implementation where `T` is unsized,
/// consider using the `RangeBounds` impl for a 2-tuple of [`Bound<&T>`][Bound],
/// i.e. replace `..=end` with `(Bound::Unbounded, Bound::Included(end))`.
#[stable(feature = "collections_range", since = "1.28.0")]
impl<T> RangeBounds<T> for RangeToInclusive<&T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end)
    }
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=13

```rust
/// An internal helper for `split_off` functions indicating
/// which end a `OneSidedRange` is bounded on.
#[unstable(feature = "one_sided_range", issue = "69780")]
#[allow(missing_debug_implementations)]
pub enum OneSidedRangeBound {
    /// The range is bounded inclusively from below and is unbounded above.
    StartInclusive,
    /// The range is bounded exclusively from above and is unbounded below.
    End,
    /// The range is bounded inclusively from above and is unbounded below.
    EndInclusive,
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=bound | COMPLEXITY=7 | LINES=13

```rust
/// `OneSidedRange` is implemented for built-in range types that are unbounded
/// on one side. For example, `a..`, `..b` and `..=c` implement `OneSidedRange`,
/// but `..`, `d..e`, and `f..=g` do not.
///
/// Types that implement `OneSidedRange<T>` must return `Bound::Unbounded`
/// from one of `RangeBounds::start_bound` or `RangeBounds::end_bound`.
#[unstable(feature = "one_sided_range", issue = "69780")]
pub trait OneSidedRange<T>: RangeBounds<T> {
    /// An internal-only helper function for `split_off` and
    /// `split_off_mut` that returns the bound of the one-sided range.
    fn bound(self) -> (OneSidedRangeBound, T);
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=bound | COMPLEXITY=5 | LINES=10

```rust
#[unstable(feature = "one_sided_range", issue = "69780")]
impl<T> OneSidedRange<T> for RangeTo<T>
where
    Self: RangeBounds<T>,
{
    fn bound(self) -> (OneSidedRangeBound, T) {
        (OneSidedRangeBound::End, self.end)
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=bound | COMPLEXITY=5 | LINES=10

```rust
#[unstable(feature = "one_sided_range", issue = "69780")]
impl<T> OneSidedRange<T> for RangeFrom<T>
where
    Self: RangeBounds<T>,
{
    fn bound(self) -> (OneSidedRangeBound, T) {
        (OneSidedRangeBound::StartInclusive, self.start)
    }
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=bound | COMPLEXITY=5 | LINES=10

```rust
#[unstable(feature = "one_sided_range", issue = "69780")]
impl<T> OneSidedRange<T> for RangeToInclusive<T>
where
    Self: RangeBounds<T>,
{
    fn bound(self) -> (OneSidedRangeBound, T) {
        (OneSidedRangeBound::EndInclusive, self.end)
    }
}
```

---
*Generated by AST tracing system*
