// Generated macro for impl_64 (impl)
macro_rules! Depcrate_rawimpl_64 {
() => {
// Module: crate::raw
// Provides: {"impl_64"}
// Dependencies: {}
impl ProbeSeq { # [inline] fn move_next (& mut self , bucket_mask : usize) { debug_assert ! (self . stride <= bucket_mask , "Went past end of probe sequence") ; self . stride += Group :: WIDTH ; self . pos += self . stride ; self . pos &= bucket_mask ; } }
};
}
