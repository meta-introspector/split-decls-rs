// Generated macro for impl_69 (impl)
macro_rules! Depcrate_softimpl_69 {
() => {
// Module: crate::soft
// Provides: {"impl_69"}
// Dependencies: {}
impl < W : StoreBytes + BSwap + Copy > StoreBytes for x4 < W > { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { let n = input . len () / 4 ; x4 ([W :: unsafe_read_le (& input [.. n]) , W :: unsafe_read_le (& input [n .. n * 2]) , W :: unsafe_read_le (& input [n * 2 .. n * 3]) , W :: unsafe_read_le (& input [n * 3 ..]) ,]) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { let n = input . len () / 4 ; x4 ([W :: unsafe_read_be (& input [.. n]) , W :: unsafe_read_be (& input [n .. n * 2]) , W :: unsafe_read_be (& input [n * 2 .. n * 3]) , W :: unsafe_read_be (& input [n * 3 ..]) ,]) } # [inline (always)] fn write_le (self , out : & mut [u8]) { let n = out . len () / 4 ; self . 0 [0] . write_le (& mut out [.. n]) ; self . 0 [1] . write_le (& mut out [n .. n * 2]) ; self . 0 [2] . write_le (& mut out [n * 2 .. n * 3]) ; self . 0 [3] . write_le (& mut out [n * 3 ..]) ; } # [inline (always)] fn write_be (self , out : & mut [u8]) { let n = out . len () / 4 ; self . 0 [0] . write_be (& mut out [.. n]) ; self . 0 [1] . write_be (& mut out [n .. n * 2]) ; self . 0 [2] . write_be (& mut out [n * 2 .. n * 3]) ; self . 0 [3] . write_be (& mut out [n * 3 ..]) ; } }
};
}
