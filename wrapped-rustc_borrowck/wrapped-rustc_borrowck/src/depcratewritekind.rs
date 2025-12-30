// Generated macro for WriteKind (enum)
macro_rules! DepcrateWriteKind {
() => {
// Module: crate
// Provides: {"WriteKind"}
// Dependencies: {}
# [doc = " Kind of write access to a value"] # [doc = " (For informational purposes only)"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum WriteKind { StorageDeadOrDrop , Replace , MutableBorrow (BorrowKind) , Mutate , Move , }
};
}
