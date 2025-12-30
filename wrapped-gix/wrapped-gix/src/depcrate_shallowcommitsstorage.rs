// Generated macro for CommitsStorage (type)
macro_rules! Depcrate_shallowCommitsStorage {
() => {
// Module: crate::shallow
// Provides: {"CommitsStorage"}
// Dependencies: {}
pub (crate) type CommitsStorage = gix_features :: threading :: OwnShared < gix_fs :: SharedFileSnapshotMut < Vec < gix_hash :: ObjectId > > > ;
};
}
