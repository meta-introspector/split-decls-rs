// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Pkcs7 { # [inline] fn unpad (block : & [u8] , strict : bool) -> Result < & [u8] , Error > { if block . len () > 255 { panic ! ("block size is too big for PKCS#7") ; } let bs = block . len () ; let n = block [bs - 1] ; if n == 0 || n as usize > bs { return Err (Error) ; } let s = bs - n as usize ; if strict && block [s .. bs - 1] . iter () . any (| & v | v != n) { return Err (Error) ; } Ok (& block [.. s]) } }
};
}
