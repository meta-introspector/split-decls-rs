// Generated macro for impl_496 (impl)
macro_rules! Depcrate_util_searchimpl_496 {
() => {
// Module: crate::util::search
// Provides: {"impl_496"}
// Dependencies: {}
impl Span { # [doc = " Returns this span as a range."] # [inline] pub fn range (& self) -> Range < usize > { Range :: from (* self) } # [doc = " Returns true when this span is empty. That is, when `start >= end`."] # [inline] pub fn is_empty (& self) -> bool { self . start >= self . end } # [doc = " Returns the length of this span."] # [doc = ""] # [doc = " This returns `0` in precisely the cases that `is_empty` returns `true`."] # [inline] pub fn len (& self) -> usize { self . end . saturating_sub (self . start) } # [doc = " Returns true when the given offset is contained within this span."] # [doc = ""] # [doc = " Note that an empty span contains no offsets and will always return"] # [doc = " false."] # [inline] pub fn contains (& self , offset : usize) -> bool { ! self . is_empty () && self . start <= offset && offset <= self . end } # [doc = " Returns a new span with `offset` added to this span's `start` and `end`"] # [doc = " values."] # [inline] pub fn offset (& self , offset : usize) -> Span { Span { start : self . start + offset , end : self . end + offset } } }
};
}
