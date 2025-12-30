// Generated macro for impl_78 (impl)
macro_rules! Depcrate_tokio_dir_builderimpl_78 {
() => {
// Module: crate::tokio::dir_builder
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (unix)] impl DirBuilder { # [doc = " Sets the mode to create new directories with."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirBuilder::mode`]."] pub fn mode (& mut self , mode : u32) -> & mut Self { self . inner . mode (mode) ; self } }
};
}
