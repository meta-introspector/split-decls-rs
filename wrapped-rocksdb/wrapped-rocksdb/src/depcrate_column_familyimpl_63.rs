// Generated macro for impl_63 (impl)
macro_rules! Depcrate_column_familyimpl_63 {
() => {
// Module: crate::column_family
// Provides: {"impl_63"}
// Dependencies: {}
impl UnboundColumnFamily { pub (crate) fn bound_column_family < 'a > (self : Arc < Self >) -> Arc < BoundColumnFamily < 'a > > { unsafe { Arc :: from_raw (Arc :: into_raw (self) . cast ()) } } }
};
}
