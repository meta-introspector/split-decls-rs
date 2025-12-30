// Generated macro for impl_196 (impl)
macro_rules! Depcrate_fileimpl_196 {
() => {
// Module: crate::file
// Provides: {"impl_196"}
// Dependencies: {}
impl From < PathBuf > for File < FileSourceFile , FileFormat > { fn from (path : PathBuf) -> Self { Self { format : None , required : true , source : FileSourceFile :: new (path) , } } }
};
}
