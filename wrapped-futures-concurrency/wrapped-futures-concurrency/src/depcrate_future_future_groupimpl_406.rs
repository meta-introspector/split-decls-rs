// Generated macro for impl_406 (impl)
macro_rules! Depcrate_future_future_groupimpl_406 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_406"}
// Dependencies: {}
impl < F : Future > FromIterator < F > for FutureGroup < F > { fn from_iter < T : IntoIterator < Item = F > > (iter : T) -> Self { let mut this = Self :: new () ; this . extend (iter) ; this } }
};
}
