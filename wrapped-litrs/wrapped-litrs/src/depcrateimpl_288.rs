// Generated macro for impl_288 (impl)
macro_rules! Depcrateimpl_288 {
() => {
// Module: crate
// Provides: {"impl_288"}
// Dependencies: {}
impl < 'a > Buffer for & 'a str { # [doc (hidden)] fn cut (self , range : Range < usize >) -> Self { & self [range] } type Cow = Cow < 'a , str > ; # [doc (hidden)] fn into_cow (self) -> Self :: Cow { self . into () } type ByteCow = Cow < 'a , [u8] > ; # [doc (hidden)] fn into_byte_cow (self) -> Self :: ByteCow { self . as_bytes () . into () } }
};
}
