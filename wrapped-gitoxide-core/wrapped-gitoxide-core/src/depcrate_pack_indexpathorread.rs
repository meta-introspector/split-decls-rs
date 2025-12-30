// Generated macro for PathOrRead (enum)
macro_rules! Depcrate_pack_indexPathOrRead {
() => {
// Module: crate::pack::index
// Provides: {"PathOrRead"}
// Dependencies: {}
pub enum PathOrRead { Path (PathBuf) , Read (Box < dyn std :: io :: Read + Send + 'static >) , }
};
}
