// Generated macro for impl_109 (impl)
macro_rules! Depcrate_rawimpl_109 {
() => {
// Module: crate::raw
// Provides: {"impl_109"}
// Dependencies: {}
impl Iterator for FullBucketsIndices { type Item = usize ; # [doc = " Advances the iterator and returns the next value. It is up to"] # [doc = " the caller to ensure that the `RawTable` outlives the `FullBucketsIndices`,"] # [doc = " because we cannot make the `next` method unsafe."] # [inline (always)] fn next (& mut self) -> Option < usize > { if self . items == 0 { return None ; } let nxt = unsafe { self . next_impl () } ; debug_assert ! (nxt . is_some ()) ; self . items -= 1 ; nxt } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { (self . items , Some (self . items)) } }
};
}
