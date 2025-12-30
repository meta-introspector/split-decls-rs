// Generated macro for impl_17 (impl)
macro_rules! Depcrate_path_extimpl_17 {
() => {
// Module: crate::path_ext
// Provides: {"impl_17"}
// Dependencies: {}
impl PathExt for PathBuf { # [inline] fn relative_to < P > (& self , root : P) -> Result < RelativePathBuf , RelativeToError > where P : AsRef < Path > , { self . as_path () . relative_to (root) } }
};
}
