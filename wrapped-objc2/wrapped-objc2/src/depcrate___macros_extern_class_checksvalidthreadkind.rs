// Generated macro for ValidThreadKind (trait)
macro_rules! Depcrate___macros_extern_class_checksValidThreadKind {
() => {
// Module: crate::__macros::extern_class::checks
// Provides: {"ValidThreadKind"}
// Dependencies: {}
# [doc = " Helper for ensuring that `ClassType::ThreadKind`, if specified, is set"] # [doc = " correctly."] pub trait ValidThreadKind < Requested : ? Sized + ThreadKind > where Self : ClassType < ThreadKind = Requested > , Self :: Super : ClassType , { fn check () { } }
};
}
