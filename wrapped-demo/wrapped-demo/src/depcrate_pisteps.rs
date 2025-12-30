// Generated macro for STEPS (const)
macro_rules! Depcrate_piSTEPS {
() => {
// Module: crate::pi
// Provides: {"STEPS"}
// Dependencies: {}
const STEPS : u64 = if cfg ! (debug_assertions) { 50_000 } else { 5_000_000 } ;
};
}
