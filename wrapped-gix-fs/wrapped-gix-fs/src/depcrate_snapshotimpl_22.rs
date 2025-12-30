// Generated macro for impl_22 (impl)
macro_rules! Depcrate_snapshotimpl_22 {
() => {
// Module: crate::snapshot
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : std :: fmt :: Debug > Deref for SharedFileSnapshotMut < T > { type Target = MutableOnDemand < Option < SharedFileSnapshot < T > > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
