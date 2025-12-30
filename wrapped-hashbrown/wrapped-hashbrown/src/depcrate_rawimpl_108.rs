// Generated macro for impl_108 (impl)
macro_rules! Depcrate_rawimpl_108 {
() => {
// Module: crate::raw
// Provides: {"impl_108"}
// Dependencies: {}
impl FullBucketsIndices { # [doc = " Advances the iterator and returns the next value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If any of the following conditions are violated, the result is"] # [doc = " [`Undefined Behavior`]:"] # [doc = ""] # [doc = " * The [`RawTableInner`] / [`RawTable`] must be alive and not moved,"] # [doc = "   i.e. table outlives the `FullBucketsIndices`;"] # [doc = ""] # [doc = " * It never tries to iterate after getting all elements."] # [doc = ""] # [doc = " [`Undefined Behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html"] # [inline (always)] unsafe fn next_impl (& mut self) -> Option < usize > { loop { if let Some (index) = self . current_group . next () { return Some (self . group_first_index + index) ; } self . ctrl = NonNull :: new_unchecked (self . ctrl . as_ptr () . add (Group :: WIDTH)) ; self . current_group = Group :: load_aligned (self . ctrl . as_ptr () . cast ()) . match_full () . into_iter () ; self . group_first_index += Group :: WIDTH ; } } }
};
}
