// Generated macro for map_walkdir_error (function)
macro_rules! Depcrate_kqueuemap_walkdir_error {
() => {
// Module: crate::kqueue
// Provides: {"map_walkdir_error"}
// Dependencies: {}
fn map_walkdir_error (e : walkdir :: Error) -> Error { if e . io_error () . is_some () { Error :: io (e . into_io_error () . unwrap ()) } else { Error :: generic (& e . to_string ()) } }
};
}
