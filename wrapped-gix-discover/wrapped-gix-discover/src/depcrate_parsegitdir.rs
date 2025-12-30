// Generated macro for gitdir (function)
macro_rules! Depcrate_parsegitdir {
() => {
// Module: crate::parse
// Provides: {"gitdir"}
// Dependencies: {}
# [doc = " Parse typical `gitdir` files as seen in worktrees and submodules."] pub fn gitdir (input : & [u8]) -> Result < PathBuf , gitdir :: Error > { let path = input . strip_prefix (b"gitdir: ") . ok_or_else (| | gitdir :: Error :: InvalidFormat { input : input . into () }) ? . as_bstr () ; let path = path . trim_end () . as_bstr () ; if path . is_empty () { return Err (gitdir :: Error :: InvalidFormat { input : input . into () }) ; } Ok (gix_path :: try_from_bstr (path) . map_err (| _ | gitdir :: Error :: IllformedUtf8 { input : input . into () }) ? . into_owned ()) }
};
}
