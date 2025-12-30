// Generated macro for InnerBufRing (struct)
macro_rules! Depcrate_tests_register_buf_ringInnerBufRing {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"InnerBufRing"}
// Dependencies: {}
pub (crate) struct InnerBufRing { bgid : Bgid , ring_entries_mask : u16 , buf_cnt : u16 , buf_len : usize , pub (crate) ring_start : AnonymousMmap , buf_list : Vec < Vec < u8 > > , local_tail : Cell < u16 > , shared_tail : * const AtomicU16 , flags : RegisterFlags , }
};
}
