// Generated macro for marker_to_path (function)
macro_rules! Depcrate_datapathmarker_to_path {
() => {
// Module: crate::datapath
// Provides: {"marker_to_path"}
// Dependencies: {}
pub (crate) fn marker_to_path (marker : DataMarkerId , root : & Path) -> PathBuf { let mut path = PathBuf :: from (root) ; let mut last = 0 ; for i in 1 .. marker . name () . len () { if marker . name () . as_bytes () . get (i + 1) . is_none_or (| b | b . is_ascii_uppercase ()) { path . push (marker . name () [last ..= i] . to_ascii_lowercase ()) ; last = i + 1 ; } } path }
};
}
