// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl FileOperation < '_ > { fn get_path (& self) -> Option < PathBuf > { match & self . basic { BasicFileOperation :: SetArchiveComment (_) => None , BasicFileOperation :: WriteDirectory (_) => Some (self . path . join ("")) , BasicFileOperation :: MergeWithOtherFile { operations , .. } => operations . iter () . flat_map (| (op , abort) | if ! abort { op . get_path () } else { None }) . next () , _ => Some (self . path . to_owned ()) , } } }
};
}
