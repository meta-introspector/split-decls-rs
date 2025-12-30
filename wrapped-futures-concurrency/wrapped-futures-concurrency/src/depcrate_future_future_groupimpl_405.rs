// Generated macro for impl_405 (impl)
macro_rules! Depcrate_future_future_groupimpl_405 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_405"}
// Dependencies: {}
impl < F : Future > Extend < F > for FutureGroup < F > { fn extend < T : IntoIterator < Item = F > > (& mut self , iter : T) { let iter = iter . into_iter () ; let len = iter . size_hint () . 1 . unwrap_or_default () ; self . reserve (len) ; for future in iter { self . insert (future) ; } } }
};
}
