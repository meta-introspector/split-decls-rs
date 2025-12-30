// Generated macro for Destination (struct)
macro_rules! Depcrate_hirDestination {
() => {
// Module: crate::hir
// Provides: {"Destination"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , HashStable_Generic)] pub struct Destination { # [doc = " This is `Some(_)` iff there is an explicit user-specified 'label"] pub label : Option < Label > , # [doc = " These errors are caught and then reported during the diagnostics pass in"] # [doc = " `librustc_passes/loops.rs`"] pub target_id : Result < HirId , LoopIdError > , }
};
}
