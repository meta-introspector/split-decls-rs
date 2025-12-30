// Generated macro for impl_163 (impl)
macro_rules! Depcrate_utf16_charimpl_163 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_163"}
// Dependencies: {}
# [cfg (feature = "std")] impl Extend < Utf16Char > for Vec < u16 > { fn extend < I : IntoIterator < Item = Utf16Char > > (& mut self , iter : I) { let iter = iter . into_iter () ; self . reserve (iter . size_hint () . 0) ; for u16c in iter { self . push (u16c . units [0]) ; if u16c . units [1] != 0 { self . push (u16c . units [1]) ; } } } }
};
}
