// Generated macro for miri_dir (function)
macro_rules! Depcrate_utilmiri_dir {
() => {
// Module: crate::util
// Provides: {"miri_dir"}
// Dependencies: {}
pub fn miri_dir () -> std :: io :: Result < PathBuf > { const MIRI_SCRIPT_ROOT_DIR : & str = env ! ("CARGO_MANIFEST_DIR") ; Ok (canonicalize (MIRI_SCRIPT_ROOT_DIR) ? . parent () . unwrap () . into ()) }
};
}
