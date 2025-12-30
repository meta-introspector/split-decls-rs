// Generated macro for tests (module)
macro_rules! Depcrate_stdtests {
() => {
// Module: crate::std
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (loom)] mod tests { use crate as critical_section ; # [cfg (feature = "std")] # [test] # [should_panic (expected = "Not a PoisonError!")] fn reusable_after_panic_loom () { loom :: model (| | { let _ = std :: thread :: spawn (| | { critical_section :: with (| _ | { panic ! ("Boom!") ; }) ; }) . join () ; critical_section :: with (| _ | { panic ! ("Not a PoisonError!") ; }) }) } }
};
}
