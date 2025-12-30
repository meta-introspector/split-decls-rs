// Generated macro for impl_92 (impl)
macro_rules! Depcrate_frontendimpl_92 {
() => {
// Module: crate::frontend
// Provides: {"impl_92"}
// Dependencies: {}
impl < B : PatternBackend > Pattern < B > { # [cfg (feature = "alloc")] pub (crate) const fn from_boxed_store_unchecked (store : Box < B :: Store >) -> Box < Self > { unsafe { core :: mem :: transmute (store) } } # [doc (hidden)] pub const fn from_ref_store_unchecked (store : & B :: Store) -> & Self { unsafe { & * (store as * const B :: Store as * const Self) } } }
};
}
