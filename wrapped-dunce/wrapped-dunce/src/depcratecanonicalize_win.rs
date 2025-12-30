// Generated macro for canonicalize_win (function)
macro_rules! Depcratecanonicalize_win {
() => {
// Module: crate
// Provides: {"canonicalize_win"}
// Dependencies: {}
# [cfg (windows)] fn canonicalize_win (path : & Path) -> io :: Result < PathBuf > { let real_path = fs :: canonicalize (path) ? ; Ok (try_simplified (& real_path) . map (PathBuf :: from) . unwrap_or (real_path)) }
};
}
