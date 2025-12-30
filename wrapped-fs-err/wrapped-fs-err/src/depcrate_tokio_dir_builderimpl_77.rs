// Generated macro for impl_77 (impl)
macro_rules! Depcrate_tokio_dir_builderimpl_77 {
() => {
// Module: crate::tokio::dir_builder
// Provides: {"impl_77"}
// Dependencies: {}
impl DirBuilder { # [doc = " Creates a new set of options with default mode/security settings for all"] # [doc = " platforms and also non-recursive."] # [doc = ""] # [doc = " This is a wrapper version of [`tokio::fs::DirBuilder::new`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use fs_err::tokio::DirBuilder;"] # [doc = ""] # [doc = " let builder = DirBuilder::new();"] # [doc = " ```"] pub fn new () -> Self { Default :: default () } # [doc = " Indicates whether to create directories recursively (including all parent"] # [doc = " directories). Parents that do not exist are created with the same security and"] # [doc = " permissions settings."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirBuilder::recursive`]."] pub fn recursive (& mut self , recursive : bool) -> & mut Self { self . inner . recursive (recursive) ; self } # [doc = " Creates the specified directory with the configured options."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::DirBuilder::create`]."] pub async fn create (& self , path : impl AsRef < Path >) -> io :: Result < () > { let path = path . as_ref () ; self . inner . create (path) . await . map_err (| err | Error :: build (err , ErrorKind :: CreateDir , path)) } }
};
}
