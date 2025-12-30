// Generated macro for rustc_path (function)
macro_rules! Depcrate_utilrustc_path {
() => {
// Module: crate::util
// Provides: {"rustc_path"}
// Dependencies: {}
pub fn rustc_path < 'a > (sysroot : & Sysroot) -> Option < & 'a Path > { static RUSTC_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ; RUSTC_PATH . get_or_init (| | { let candidate = sysroot . default . join (env ! ("RUSTC_INSTALL_BINDIR")) . join (if cfg ! (target_os = "windows") { "rustc.exe" } else { "rustc" }) ; candidate . exists () . then_some (candidate) }) . as_deref () }
};
}
