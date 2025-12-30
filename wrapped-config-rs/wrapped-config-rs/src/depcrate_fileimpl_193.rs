// Generated macro for impl_193 (impl)
macro_rules! Depcrate_fileimpl_193 {
() => {
// Module: crate::file
// Provides: {"impl_193"}
// Dependencies: {}
impl File < FileSourceFile , FileFormat > { # [doc = " Given the basename of a file, will attempt to locate a file by setting its"] # [doc = " extension to a registered format."] pub fn with_name (base_name : & str) -> Self { Self { format : None , required : true , source : FileSourceFile :: new (base_name . into ()) , } } }
};
}
