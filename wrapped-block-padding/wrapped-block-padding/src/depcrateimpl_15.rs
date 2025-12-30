// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl Padding for Iso7816 { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { if pos >= block . len () { panic ! ("`pos` is bigger or equal to block size") ; } block [pos] = 0x80 ; block [pos + 1 ..] . fill (0) ; } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { for i in (0 .. block . len ()) . rev () { match block [i] { 0x80 => return Ok (& block [.. i]) , 0x00 => continue , _ => return Err (Error) , } } Err (Error) } }
};
}
