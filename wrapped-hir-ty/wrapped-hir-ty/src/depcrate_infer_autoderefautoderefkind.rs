// Generated macro for AutoderefKind (enum)
macro_rules! Depcrate_infer_autoderefAutoderefKind {
() => {
// Module: crate::infer::autoderef
// Provides: {"AutoderefKind"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub (crate) enum AutoderefKind { # [doc = " A true pointer type, such as `&T` and `*mut T`."] Builtin , # [doc = " A type which must dispatch to a `Deref` implementation."] Overloaded , }
};
}
