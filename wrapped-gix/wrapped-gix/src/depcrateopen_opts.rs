// Generated macro for open_opts (function)
macro_rules! Depcrateopen_opts {
() => {
// Module: crate
// Provides: {"open_opts"}
// Dependencies: {}
# [doc = " See [`ThreadSafeRepository::open_opts()`], but returns a [`Repository`] instead."] # [allow (clippy :: result_large_err)] # [doc (alias = "open_ext" , alias = "git2")] pub fn open_opts (directory : impl Into < std :: path :: PathBuf > , options : open :: Options) -> Result < Repository , open :: Error > { ThreadSafeRepository :: open_opts (directory , options) . map (Into :: into) }
};
}
