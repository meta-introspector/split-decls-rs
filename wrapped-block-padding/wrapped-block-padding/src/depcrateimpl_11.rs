// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Padding for Iso10126 { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { Pkcs7 :: raw_pad (block , pos) } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { Pkcs7 :: unpad (block , false) } }
};
}
