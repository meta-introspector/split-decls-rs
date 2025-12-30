// Generated macro for impl_42 (impl)
macro_rules! Depcrate_decode_value_refimpl_42 {
() => {
// Module: crate::decode::value_ref
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > BorrowRead < 'a > for & 'a [u8] { fn fill_buf (& self) -> & 'a [u8] { self } fn consume (& mut self , len : usize) { * self = & (* self) [len ..] ; } }
};
}
