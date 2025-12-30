// Generated macro for resolve_binary (function)
macro_rules! Depcrate_decompressresolve_binary {
() => {
// Module: crate::decompress
// Provides: {"resolve_binary"}
// Dependencies: {}
# [doc = " Resolves a path to a program to a path by searching for the program in"] # [doc = " `PATH`."] # [doc = ""] # [doc = " If the program could not be resolved, then an error is returned."] # [doc = ""] # [doc = " The purpose of doing this instead of passing the path to the program"] # [doc = " directly to Command::new is that Command::new will hand relative paths"] # [doc = " to CreateProcess on Windows, which will implicitly search the current"] # [doc = " working directory for the executable. This could be undesirable for"] # [doc = " security reasons. e.g., running ripgrep with the -z/--search-zip flag on an"] # [doc = " untrusted directory tree could result in arbitrary programs executing on"] # [doc = " Windows."] # [doc = ""] # [doc = " Note that this could still return a relative path if PATH contains a"] # [doc = " relative path. We permit this since it is assumed that the user has set"] # [doc = " this explicitly, and thus, desires this behavior."] # [doc = ""] # [doc = " # Platform behavior"] # [doc = ""] # [doc = " On non-Windows, this is a no-op."] pub fn resolve_binary < P : AsRef < Path > > (prog : P ,) -> Result < PathBuf , CommandError > { if ! cfg ! (windows) { return Ok (prog . as_ref () . to_path_buf ()) ; } try_resolve_binary (prog) }
};
}
