// Generated macro for impl_296 (impl)
macro_rules! Depcrate_extensions_unicode_valueimpl_296 {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"impl_296"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl Extend < Subtag > for Value { fn extend < T : IntoIterator < Item = Subtag > > (& mut self , iter : T) { for i in iter { self . 0 . push (i) ; } } }
};
}
