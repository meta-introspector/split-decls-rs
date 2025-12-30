// Generated macro for impl_91 (impl)
macro_rules! Depcrate_frontendimpl_91 {
() => {
// Module: crate::frontend
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < B : PatternBackend > Clone for Box < Pattern < B > > where Box < B :: Store > : for < 'a > From < & 'a B :: Store > , { fn clone (& self) -> Self { Pattern :: from_boxed_store_unchecked (Box :: from (& self . store)) } }
};
}
