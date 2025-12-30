// Generated macro for maybe_sync_file (function)
macro_rules! Depcrate_shims_unix_fsmaybe_sync_file {
() => {
// Module: crate::shims::unix::fs
// Provides: {"maybe_sync_file"}
// Dependencies: {}
fn maybe_sync_file (file : & File , writable : bool , operation : fn (& File) -> std :: io :: Result < () > ,) -> std :: io :: Result < i32 > { if ! writable && cfg ! (windows) { Ok (0i32) } else { let result = operation (file) ; result . map (| _ | 0i32) } }
};
}
