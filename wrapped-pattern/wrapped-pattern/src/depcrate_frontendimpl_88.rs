// Generated macro for impl_88 (impl)
macro_rules! Depcrate_frontendimpl_88 {
() => {
// Module: crate::frontend
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < B : PatternBackend > Default for Box < Pattern < B > > where Box < B :: Store > : From < & 'static B :: Store > , { fn default () -> Self { Pattern :: from_boxed_store_unchecked (Box :: from (B :: empty ())) } }
};
}
