// Generated macro for unsupported_int_any (macro)
macro_rules! Depcrate_numunsupported_int_any {
() => {
// Module: crate::num
// Provides: {"unsupported_int_any"}
// Dependencies: {}
# [cfg (not (target_pointer_width = "64"))] macro_rules ! unsupported_int_any { ($ runner : ident , $ typ : ty) => { $ runner . rng () . next_u32 () as $ typ } ; }
};
}
