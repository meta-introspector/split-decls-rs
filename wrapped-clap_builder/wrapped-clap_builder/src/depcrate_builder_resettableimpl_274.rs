// Generated macro for impl_274 (impl)
macro_rules! Depcrate_builder_resettableimpl_274 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_274"}
// Dependencies: {}
impl < I : Into < OsStr > > IntoResettable < OsStr > for I { fn into_resettable (self) -> Resettable < OsStr > { Resettable :: Value (self . into ()) } }
};
}
