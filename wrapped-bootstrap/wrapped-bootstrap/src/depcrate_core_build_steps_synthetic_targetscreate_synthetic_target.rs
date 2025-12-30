// Generated macro for create_synthetic_target (function)
macro_rules! Depcrate_core_build_steps_synthetic_targetscreate_synthetic_target {
() => {
// Module: crate::core::build_steps::synthetic_targets
// Provides: {"create_synthetic_target"}
// Dependencies: {}
fn create_synthetic_target (builder : & Builder < '_ > , compiler : Compiler , suffix : & str , base : TargetSelection , customize : impl FnOnce (& mut serde_json :: Map < String , serde_json :: Value >) ,) -> TargetSelection { if base . contains ("synthetic") { panic ! ("cannot create synthetic targets with other synthetic targets as their base") ; } let name = format ! ("{base}-synthetic-{suffix}") ; let path = builder . out . join ("synthetic-target-specs") . join (format ! ("{name}.json")) ; std :: fs :: create_dir_all (path . parent () . unwrap ()) . unwrap () ; if builder . config . dry_run () { std :: fs :: write (& path , b"dry run\n") . unwrap () ; return TargetSelection :: create_synthetic (& name , path . to_str () . unwrap ()) ; } let mut cmd = builder . rustc_cmd (compiler) ; cmd . arg ("--target") . arg (base . rustc_target_arg ()) ; cmd . args (["-Zunstable-options" , "--print" , "target-spec-json"]) ; cmd . env ("RUSTC_BOOTSTRAP" , "1") ; let output = cmd . run_capture (builder) . stdout () ; let mut spec : serde_json :: Value = serde_json :: from_slice (output . as_bytes ()) . unwrap () ; let spec_map = spec . as_object_mut () . unwrap () ; spec_map . remove ("is-builtin") ; customize (spec_map) ; std :: fs :: write (& path , serde_json :: to_vec_pretty (& spec) . unwrap ()) . unwrap () ; TargetSelection :: create_synthetic (& name , path . to_str () . unwrap ()) }
};
}
