// Generated macro for impl_90 (impl)
macro_rules! Depcrate_frontendimpl_90 {
() => {
// Module: crate::frontend
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < B : PatternBackend > ToOwned for Pattern < B > where Box < B :: Store > : for < 'a > From < & 'a B :: Store > , { type Owned = Box < Pattern < B > > ; fn to_owned (& self) -> Self :: Owned { Self :: from_boxed_store_unchecked (Box :: from (& self . store)) } }
};
}
