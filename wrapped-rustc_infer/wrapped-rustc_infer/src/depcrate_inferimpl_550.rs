// Generated macro for impl_550 (impl)
macro_rules! Depcrate_inferimpl_550 {
() => {
// Module: crate::infer
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'tcx > ValuePairs < 'tcx > { pub fn ty (& self) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { if let ValuePairs :: Terms (ExpectedFound { expected , found }) = self && let Some (expected) = expected . as_type () && let Some (found) = found . as_type () { Some ((expected , found)) } else { None } } }
};
}
