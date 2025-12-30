// Generated macro for ManuallyDropThroughUnion (enum)
macro_rules! Depcrate_referenceManuallyDropThroughUnion {
() => {
// Module: crate::reference
// Provides: {"ManuallyDropThroughUnion"}
// Dependencies: {}
# [doc = " Is this a `ManuallyDrop` reached through a union, and when is `DerefMut` called on it?"] enum ManuallyDropThroughUnion { # [doc = " `ManuallyDrop` reached through a union and immediately explicitely dereferenced"] Directly , # [doc = " `ManuallyDrop` reached through a union, and dereferenced later on"] Indirect , # [doc = " Any other situation"] No , }
};
}
