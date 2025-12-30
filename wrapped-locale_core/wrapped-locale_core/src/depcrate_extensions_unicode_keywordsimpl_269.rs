// Generated macro for impl_269 (impl)
macro_rules! Depcrate_extensions_unicode_keywordsimpl_269 {
() => {
// Module: crate::extensions::unicode::keywords
// Provides: {"impl_269"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromIterator < (Key , Value) > for Keywords { fn from_iter < I : IntoIterator < Item = (Key , Value) > > (iter : I) -> Self { LiteMap :: from_iter (iter) . into () } }
};
}
