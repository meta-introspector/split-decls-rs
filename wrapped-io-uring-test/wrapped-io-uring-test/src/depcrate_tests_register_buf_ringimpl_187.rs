// Generated macro for impl_187 (impl)
macro_rules! Depcrate_tests_register_buf_ringimpl_187 {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"impl_187"}
// Dependencies: {}
impl GBuf { fn new (bufgroup : FixedSizeBufRing , bid : Bid , len : usize) -> Self { assert ! (len <= bufgroup . rc . buf_len) ; Self { bufgroup , len , bid } } # [allow (dead_code)] fn len (& self) -> usize { self . len as _ } # [allow (dead_code)] fn is_empty (& self) -> bool { self . len () == 0 } # [allow (dead_code)] fn cap (& self) -> usize { self . bufgroup . rc . buf_capacity () } pub (crate) fn as_slice (& self) -> & [u8] { let p = self . bufgroup . rc . stable_ptr (self . bid) ; unsafe { std :: slice :: from_raw_parts (p , self . len) } } }
};
}
