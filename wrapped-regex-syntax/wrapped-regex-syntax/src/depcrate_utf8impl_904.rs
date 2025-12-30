// Generated macro for impl_904 (impl)
macro_rules! Depcrate_utf8impl_904 {
() => {
// Module: crate::utf8
// Provides: {"impl_904"}
// Dependencies: {}
impl Utf8Range { fn new (start : u8 , end : u8) -> Self { Utf8Range { start , end } } # [doc = " Returns true if and only if the given byte is in this range."] pub fn matches (& self , b : u8) -> bool { self . start <= b && b <= self . end } }
};
}
