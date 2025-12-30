// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Padding for Pkcs7 { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { if block . len () > 255 { panic ! ("block size is too big for PKCS#7") ; } if pos >= block . len () { panic ! ("`pos` is bigger or equal to block size") ; } let n = (block . len () - pos) as u8 ; block [pos ..] . fill (n) ; } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { Pkcs7 :: unpad (block , true) } }
};
}
