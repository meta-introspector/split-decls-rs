// Generated macro for StorageDeadOrDrop (enum)
macro_rules! Depcrate_diagnostics_conflict_errorsStorageDeadOrDrop {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"StorageDeadOrDrop"}
// Dependencies: {}
# [doc = " Which case a StorageDeadOrDrop is for."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum StorageDeadOrDrop < 'tcx > { LocalStorageDead , BoxedStorageDead , Destructor (Ty < 'tcx >) , }
};
}
