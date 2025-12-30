// Generated macro for impl_78 (impl)
macro_rules! Depcrate_utf8_charimpl_78 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "std")] impl Extend < Utf8Char > for Vec < u8 > { fn extend < I : IntoIterator < Item = Utf8Char > > (& mut self , iter : I) { let iter = iter . into_iter () ; self . reserve (iter . size_hint () . 0) ; for u8c in iter { self . push (u8c . bytes [0]) ; for & extra in & u8c . bytes [1 ..] { if extra != 0 { self . push (extra) ; } } } } }
};
}
