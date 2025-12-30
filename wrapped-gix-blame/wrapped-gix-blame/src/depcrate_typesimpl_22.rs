// Generated macro for impl_22 (impl)
macro_rules! Depcrate_typesimpl_22 {
() => {
// Module: crate::types
// Provides: {"impl_22"}
// Dependencies: {}
impl Offset { # [doc = " Shift the given `range` according to our offset."] pub fn shifted_range (& self , range : & Range < u32 >) -> Range < u32 > { match self { Offset :: Added (added) => { debug_assert ! (range . start >= * added , "{self:?} {range:?}") ; Range { start : range . start - added , end : range . end - added , } } Offset :: Deleted (deleted) => Range { start : range . start + deleted , end : range . end + deleted , } , } } }
};
}
