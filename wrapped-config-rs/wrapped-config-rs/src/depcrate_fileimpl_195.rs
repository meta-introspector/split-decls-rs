// Generated macro for impl_195 (impl)
macro_rules! Depcrate_fileimpl_195 {
() => {
// Module: crate::file
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'a > From < & 'a Path > for File < FileSourceFile , FileFormat > { fn from (path : & 'a Path) -> Self { Self { format : None , required : true , source : FileSourceFile :: new (path . to_path_buf ()) , } } }
};
}
