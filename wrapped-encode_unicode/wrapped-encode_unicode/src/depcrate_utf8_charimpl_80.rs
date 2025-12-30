// Generated macro for impl_80 (impl)
macro_rules! Depcrate_utf8_charimpl_80 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "std")] impl Extend < Utf8Char > for String { fn extend < I : IntoIterator < Item = Utf8Char > > (& mut self , iter : I) { unsafe { self . as_mut_vec () . extend (iter) } } }
};
}
