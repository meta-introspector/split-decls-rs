// Generated macro for impl_184 (impl)
macro_rules! Depcrate_tests_register_buf_ringimpl_184 {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"impl_184"}
// Dependencies: {}
impl Builder { pub (crate) fn new (bgid : Bgid) -> Builder { Builder { bgid , ring_entries : 128 , buf_cnt : 0 , buf_len : 4096 , flags : 0 , } } pub (crate) fn register_flags (mut self , flags : RegisterFlags) -> Builder { self . flags = flags ; self } pub (crate) fn ring_entries (mut self , ring_entries : u16) -> Builder { self . ring_entries = ring_entries ; self } pub (crate) fn buf_cnt (mut self , buf_cnt : u16) -> Builder { self . buf_cnt = buf_cnt ; self } pub (crate) fn buf_len (mut self , buf_len : usize) -> Builder { self . buf_len = buf_len ; self } pub (crate) fn build (& self) -> io :: Result < FixedSizeBufRing > { let mut b : Builder = * self ; if b . buf_cnt == 0 || b . ring_entries < b . buf_cnt { let max = std :: cmp :: max (b . ring_entries , b . buf_cnt) ; b . buf_cnt = max ; b . ring_entries = max ; } if b . ring_entries > (1 << 15) { return Err (io :: Error :: new (io :: ErrorKind :: Other , "ring_entries exceeded 32768" ,)) ; } b . ring_entries = b . ring_entries . next_power_of_two () ; let inner = InnerBufRing :: new (b . bgid , b . ring_entries , b . buf_cnt , b . buf_len , b . flags) ? ; Ok (FixedSizeBufRing :: new (inner)) } }
};
}
