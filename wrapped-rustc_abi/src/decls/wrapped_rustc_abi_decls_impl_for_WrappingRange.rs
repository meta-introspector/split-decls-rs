use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl WrappingRange {
    pub fn full(size: Size) -> Self {
        Self {
            start: 0,
            end: size.unsigned_int_max(),
        }
    }
    /// Returns `true` if `v` is contained in the range.
    #[inline(always)]
    pub fn contains(&self, v: u128) -> bool {
        if self.start <= self.end {
            self.start <= v && v <= self.end
        } else {
            self.start <= v || v <= self.end
        }
    }
    /// Returns `true` if all the values in `other` are contained in this range,
    /// when the values are considered as having width `size`.
    #[inline(always)]
    pub fn contains_range(&self, other: Self, size: Size) -> bool {
        if self.is_full_for(size) {
            true
        } else {
            let trunc = |x| size.truncate(x);
            let delta = self.start;
            let max = trunc(self.end.wrapping_sub(delta));
            let other_start = trunc(other.start.wrapping_sub(delta));
            let other_end = trunc(other.end.wrapping_sub(delta));
            (other_start <= other_end) && (other_end <= max)
        }
    }
    /// Returns `self` with replaced `start`
    #[inline(always)]
    fn with_start(mut self, start: u128) -> Self {
        self.start = start;
        self
    }
    /// Returns `self` with replaced `end`
    #[inline(always)]
    fn with_end(mut self, end: u128) -> Self {
        self.end = end;
        self
    }
    /// Returns `true` if `size` completely fills the range.
    ///
    /// Note that this is *not* the same as `self == WrappingRange::full(size)`.
    /// Niche calculations can produce full ranges which are not the canonical one;
    /// for example `Option<NonZero<u16>>` gets `valid_range: (..=0) | (1..)`.
    #[inline]
    fn is_full_for(&self, size: Size) -> bool {
        let max_value = size.unsigned_int_max();
        debug_assert!(self.start <= max_value && self.end <= max_value);
        self.start == (self.end.wrapping_add(1) & max_value)
    }
    /// Checks whether this range is considered non-wrapping when the values are
    /// interpreted as *unsigned* numbers of width `size`.
    ///
    /// Returns `Ok(true)` if there's no wrap-around, `Ok(false)` if there is,
    /// and `Err(..)` if the range is full so it depends how you think about it.
    #[inline]
    pub fn no_unsigned_wraparound(&self, size: Size) -> Result<bool, RangeFull> {
        if self.is_full_for(size) { Err(..) } else { Ok(self.start <= self.end) }
    }
    /// Checks whether this range is considered non-wrapping when the values are
    /// interpreted as *signed* numbers of width `size`.
    ///
    /// This is heavily dependent on the `size`, as `100..=200` does wrap when
    /// interpreted as `i8`, but doesn't when interpreted as `i16`.
    ///
    /// Returns `Ok(true)` if there's no wrap-around, `Ok(false)` if there is,
    /// and `Err(..)` if the range is full so it depends how you think about it.
    #[inline]
    pub fn no_signed_wraparound(&self, size: Size) -> Result<bool, RangeFull> {
        if self.is_full_for(size) {
            Err(..)
        } else {
            let start: i128 = size.sign_extend(self.start);
            let end: i128 = size.sign_extend(self.end);
            Ok(start <= end)
        }
    }
}
