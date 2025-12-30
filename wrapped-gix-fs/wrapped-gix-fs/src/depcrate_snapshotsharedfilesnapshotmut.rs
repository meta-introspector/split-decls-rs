// Generated macro for SharedFileSnapshotMut (struct)
macro_rules! Depcrate_snapshotSharedFileSnapshotMut {
() => {
// Module: crate::snapshot
// Provides: {"SharedFileSnapshotMut"}
// Dependencies: {}
# [doc = " Use this type for fields in structs that are to store the [`FileSnapshot`], typically behind an [`OwnShared`]."] # [doc = ""] # [doc = " Note that the resource itself is behind another [`OwnShared`] to allow it to be used without holding any kind of lock, hence"] # [doc = " without blocking updates while it is used."] # [derive (Debug , Default)] pub struct SharedFileSnapshotMut < T : std :: fmt :: Debug > (pub MutableOnDemand < Option < SharedFileSnapshot < T > > >) ;
};
}
