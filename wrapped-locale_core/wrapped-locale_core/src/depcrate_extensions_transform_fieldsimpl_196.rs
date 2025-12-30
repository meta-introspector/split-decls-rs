// Generated macro for impl_196 (impl)
macro_rules! Depcrate_extensions_transform_fieldsimpl_196 {
() => {
// Module: crate::extensions::transform::fields
// Provides: {"impl_196"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl core :: iter :: FromIterator < (Key , Value) > for Fields { fn from_iter < I : IntoIterator < Item = (Key , Value) > > (iter : I) -> Self { LiteMap :: from_iter (iter) . into () } }
};
}
