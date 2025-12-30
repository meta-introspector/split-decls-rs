// Generated macro for impl_26 (impl)
macro_rules! Depcrate_control_tagimpl_26 {
() => {
// Module: crate::control::tag
// Provides: {"impl_26"}
// Dependencies: {}
impl TagSliceExt for [Tag] { # [inline] fn fill_tag (& mut self , tag : Tag) { unsafe { self . as_mut_ptr () . write_bytes (tag . 0 , self . len ()) } } }
};
}
