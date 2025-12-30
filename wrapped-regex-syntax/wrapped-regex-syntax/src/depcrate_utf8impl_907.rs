// Generated macro for impl_907 (impl)
macro_rules! Depcrate_utf8impl_907 {
() => {
// Module: crate::utf8
// Provides: {"impl_907"}
// Dependencies: {}
impl Utf8Sequences { # [doc = " Create a new iterator over UTF-8 byte ranges for the scalar value range"] # [doc = " given."] pub fn new (start : char , end : char) -> Self { let range = ScalarRange { start : u32 :: from (start) , end : u32 :: from (end) } ; Utf8Sequences { range_stack : vec ! [range] } } # [doc = " reset resets the scalar value range."] # [doc = " Any existing state is cleared, but resources may be reused."] # [doc = ""] # [doc = " N.B. Benchmarks say that this method is dubious."] # [doc (hidden)] pub fn reset (& mut self , start : char , end : char) { self . range_stack . clear () ; self . push (u32 :: from (start) , u32 :: from (end)) ; } fn push (& mut self , start : u32 , end : u32) { self . range_stack . push (ScalarRange { start , end }) ; } }
};
}
