// Generated macro for impl_43 (impl)
macro_rules! Depcrate_decode_value_refimpl_43 {
() => {
// Module: crate::decode::value_ref
// Provides: {"impl_43"}
// Dependencies: {}
# [doc = " Useful when you want to know how much bytes has been consumed during `ValueRef` decoding."] impl < 'a > BorrowRead < 'a > for Cursor < & 'a [u8] > { fn fill_buf (& self) -> & 'a [u8] { let len = std :: cmp :: min (self . position () , self . get_ref () . len () as u64) ; & self . get_ref () [len as usize ..] } fn consume (& mut self , len : usize) { let pos = self . position () ; self . set_position (pos + len as u64) ; } }
};
}
