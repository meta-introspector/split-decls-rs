// Generated macro for clean (function)
macro_rules! Depcrate_core_build_steps_cleanclean {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"clean"}
// Dependencies: {}
fn clean (build : & Build , all : bool , stage : Option < u32 >) { if build . config . dry_run () { return ; } rm_rf ("tmp" . as_ref ()) ; if all { rm_rf (& build . out) ; return ; } if let Some (stage) = stage { clean_specific_stage (build , stage) ; return ; } clean_default (build) ; }
};
}
