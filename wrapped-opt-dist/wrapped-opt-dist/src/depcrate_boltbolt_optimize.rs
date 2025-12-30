// Generated macro for bolt_optimize (function)
macro_rules! Depcrate_boltbolt_optimize {
() => {
// Module: crate::bolt
// Provides: {"bolt_optimize"}
// Dependencies: {}
# [doc = " Optimizes the file at `path` with BOLT in-place using the given `profile`."] pub fn bolt_optimize (path : & Utf8Path , profile : & BoltProfile , env : & Environment ,) -> anyhow :: Result < () > { let temp_path = tempfile :: NamedTempFile :: new () ? . into_temp_path () ; copy_file (path , & temp_path) ? ; let split_strategy = if env . host_tuple () . starts_with ("aarch64") { "profile2" } else { "cdsplit" } ; cmd (& [env . llvm_bolt () . as_str ()]) . arg (temp_path . display ()) . arg ("-data") . arg (& profile . 0) . arg ("-o") . arg (path) . arg ("-reorder-blocks=ext-tsp") . arg ("-reorder-functions=cdsort") . arg ("-split-functions") . arg (format ! ("-split-strategy={split_strategy}")) . arg ("-split-all-cold") . arg ("-jump-tables=move") . arg ("-icf=all") . arg ("-update-debug-sections") . arg ("-dyno-stats") . run () . with_context (| | anyhow :: anyhow ! ("Could not optimize {path} with BOLT")) ? ; Ok (()) }
};
}
