// Generated macro for impl_295 (impl)
macro_rules! Depcrate_extensions_unicode_valueimpl_295 {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"impl_295"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl FromIterator < Subtag > for Value { fn from_iter < T : IntoIterator < Item = Subtag > > (iter : T) -> Self { Self (ShortBoxSlice :: from_iter (iter)) } }
};
}
