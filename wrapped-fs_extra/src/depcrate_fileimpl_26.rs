// Generated macro for impl_26 (impl)
macro_rules! Depcrate_fileimpl_26 {
() => {
// Module: crate::file
// Provides: {"impl_26"}
// Dependencies: {}
impl CopyOptions { # [doc = " Initialize struct CopyOptions with default value."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = ""] # [doc = " overwrite: false"] # [doc = ""] # [doc = " skip_exist: false"] # [doc = ""] # [doc = " buffer_size: 64000 //64kb"] # [doc = " ```"] pub fn new () -> CopyOptions { CopyOptions { overwrite : false , skip_exist : false , buffer_size : 64000 , } } # [doc = " Sets the option true for overwrite existing files."] pub fn overwrite (mut self , overwrite : bool) -> Self { self . overwrite = overwrite ; self } # [doc = " Sets the option true for skip existing files."] pub fn skip_exist (mut self , skip_exist : bool) -> Self { self . skip_exist = skip_exist ; self } # [doc = " Sets buffer size for copy/move work only with receipt information about process work."] pub fn buffer_size (mut self , buffer_size : usize) -> Self { self . buffer_size = buffer_size ; self } }
};
}
