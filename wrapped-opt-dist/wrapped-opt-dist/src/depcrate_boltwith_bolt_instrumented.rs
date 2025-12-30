// Generated macro for with_bolt_instrumented (function)
macro_rules! Depcrate_boltwith_bolt_instrumented {
() => {
// Module: crate::bolt
// Provides: {"with_bolt_instrumented"}
// Dependencies: {}
# [doc = " Instruments an artifact at the given `path` (in-place) with BOLT and then calls `func`."] # [doc = " After this function finishes, the original file will be restored."] pub fn with_bolt_instrumented < F : FnOnce (& Utf8Path) -> anyhow :: Result < R > , R > (env : & Environment , path : & Utf8Path , func : F ,) -> anyhow :: Result < R > { let _backup_file = BackedUpFile :: new (path) ? ; let instrumented_path = tempfile :: NamedTempFile :: new () ? . into_temp_path () ; let profile_dir = tempfile :: TempDir :: new () . context ("Could not create directory for BOLT profiles") ? ; let profile_prefix = profile_dir . path () . join ("prof.fdata") ; let profile_prefix = Utf8Path :: from_path (& profile_prefix) . unwrap () ; cmd (& [env . llvm_bolt () . as_str ()]) . arg ("-instrument") . arg (path) . arg (& format ! ("--instrumentation-file={profile_prefix}")) . arg ("--instrumentation-file-append-pid") . arg ("-o") . arg (instrumented_path . display ()) . run () . with_context (| | anyhow :: anyhow ! ("Could not instrument {path} using BOLT")) ? ; copy_file (& instrumented_path , path) ? ; func (profile_prefix) }
};
}
