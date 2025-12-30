// Generated macro for impl_98 (impl)
macro_rules! Depcrate_rawimpl_98 {
() => {
// Module: crate::raw
// Provides: {"impl_98"}
// Dependencies: {}
impl < T > Iterator for RawIterRange < T > { type Item = Bucket < T > ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < Bucket < T > > { unsafe { self . next_impl :: < true > () } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let remaining_buckets = if self . end > self . next_ctrl { unsafe { offset_from (self . end , self . next_ctrl) } } else { 0 } ; (0 , Some (Group :: WIDTH + remaining_buckets)) } }
};
}
