// Generated macro for TypeAscriptionKind (enum)
macro_rules! Depcrate_mutex_atomicTypeAscriptionKind {
() => {
// Module: crate::mutex_atomic
// Provides: {"TypeAscriptionKind"}
// Dependencies: {}
# [doc = " Whether the type ascription `: Mutex<X>` (which we'll suggest replacing with `AtomicX`) is"] # [doc = " required"] enum TypeAscriptionKind < 'tcx > { # [doc = " Yes; for us, this is the case for statics"] Required (& 'tcx hir :: Ty < 'tcx >) , # [doc = " No; the ascription might've been necessary in an expression like:"] # [doc = " ```ignore"] # [doc = " let mutex: Mutex<u64> = Mutex::new(0);"] # [doc = " ```"] # [doc = " to specify the type of `0`, but since `AtomicX` already refers to a concrete type, we won't"] # [doc = " need this ascription anymore."] Optional (Option < & 'tcx hir :: Ty < 'tcx > >) , }
};
}
