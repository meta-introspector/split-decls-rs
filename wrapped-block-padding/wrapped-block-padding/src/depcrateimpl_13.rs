// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl Padding for AnsiX923 { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { if block . len () > 255 { panic ! ("block size is too big for ANSI X9.23") ; } if pos >= block . len () { panic ! ("`pos` is bigger or equal to block size") ; } let bs = block . len () ; block [pos .. bs - 1] . fill (0) ; block [bs - 1] = (bs - pos) as u8 ; } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { if block . len () > 255 { panic ! ("block size is too big for ANSI X9.23") ; } let bs = block . len () ; let n = block [bs - 1] as usize ; if n == 0 || n > bs { return Err (Error) ; } let s = bs - n ; if block [s .. bs - 1] . iter () . any (| & v | v != 0) { return Err (Error) ; } Ok (& block [.. s]) } }
};
}
