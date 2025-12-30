// Generated macro for impl_278 (impl)
macro_rules! Depcrateimpl_278 {
() => {
// Module: crate
// Provides: {"impl_278"}
// Dependencies: {}
impl MuBuilder { fn new (tr : & [u8] , ctx : & [u8]) -> Self { let mut h = H :: default () ; h = h . absorb (tr) ; h = h . absorb (& [0]) ; h = h . absorb (& [Truncate :: truncate (ctx . len ())]) ; h = h . absorb (ctx) ; Self (h) } fn internal (tr : & [u8] , Mp : & [& [u8]]) -> B64 { let mut h = H :: default () . absorb (tr) ; for m in Mp { h = h . absorb (m) ; } h . squeeze_new () } fn message (mut self , M : & [& [u8]]) -> B64 { for m in M { self . 0 = self . 0 . absorb (m) ; } self . 0 . squeeze_new () } fn finish (mut self) -> B64 { self . 0 . squeeze_new () } }
};
}
