// Generated macro for GIT_CORE_DIR (static)
macro_rules! Depcrate_envGIT_CORE_DIR {
() => {
// Module: crate::env
// Provides: {"GIT_CORE_DIR"}
// Dependencies: {}
static GIT_CORE_DIR : LazyLock < Option < PathBuf > > = LazyLock :: new (| | { let mut cmd = std :: process :: Command :: new (exe_invocation ()) ; # [cfg (windows)] { use std :: os :: windows :: process :: CommandExt ; const CREATE_NO_WINDOW : u32 = 0x08000000 ; cmd . creation_flags (CREATE_NO_WINDOW) ; } let output = cmd . arg ("--exec-path") . output () . ok () ? ; if ! output . status . success () { return None ; } BString :: new (output . stdout) . strip_suffix (b"\n") ? . to_path () . ok () ? . to_owned () . into () }) ;
};
}
