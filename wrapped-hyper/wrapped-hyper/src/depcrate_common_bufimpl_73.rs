// Generated macro for impl_73 (impl)
macro_rules! Depcrate_common_bufimpl_73 {
() => {
// Module: crate::common::buf
// Provides: {"impl_73"}
// Dependencies: {}
impl < T : Buf > BufList < T > { pub (crate) fn new () -> BufList < T > { BufList { bufs : VecDeque :: new () , } } # [inline] pub (crate) fn push (& mut self , buf : T) { debug_assert ! (buf . has_remaining ()) ; self . bufs . push_back (buf) ; } # [inline] pub (crate) fn bufs_cnt (& self) -> usize { self . bufs . len () } }
};
}
