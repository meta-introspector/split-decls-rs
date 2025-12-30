// Generated macro for clean_specific_stage (function)
macro_rules! Depcrate_core_build_steps_cleanclean_specific_stage {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"clean_specific_stage"}
// Dependencies: {}
fn clean_specific_stage (build : & Build , stage : u32) { for host in & build . hosts { let entries = match build . out . join (host) . read_dir () { Ok (iter) => iter , Err (_) => continue , } ; for entry in entries { let entry = t ! (entry) ; let stage_prefix = format ! ("stage{}" , stage + 1) ; if ! entry . file_name () . to_str () . unwrap_or ("") . contains (& stage_prefix) { continue ; } let path = t ! (entry . path () . canonicalize ()) ; rm_rf (& path) ; } } }
};
}
