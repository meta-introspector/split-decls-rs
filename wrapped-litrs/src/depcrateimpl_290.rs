// Generated macro for impl_290 (impl)
macro_rules! Depcrateimpl_290 {
() => {
// Module: crate
// Provides: {"impl_290"}
// Dependencies: {}
impl Buffer for String { # [doc (hidden)] fn cut (mut self , range : Range < usize >) -> Self { self . truncate (range . end) ; self . drain (.. range . start) ; self } type Cow = String ; # [doc (hidden)] fn into_cow (self) -> Self :: Cow { self } type ByteCow = Vec < u8 > ; # [doc (hidden)] fn into_byte_cow (self) -> Self :: ByteCow { self . into_bytes () } }
};
}
