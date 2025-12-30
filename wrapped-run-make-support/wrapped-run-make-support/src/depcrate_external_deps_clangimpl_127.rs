// Generated macro for impl_127 (impl)
macro_rules! Depcrate_external_deps_clangimpl_127 {
() => {
// Module: crate::external_deps::clang
// Provides: {"impl_127"}
// Dependencies: {}
impl Clang { # [doc = " Construct a new `clang` invocation. `clang` is not always available for all targets."] # [track_caller] pub fn new () -> Self { let clang = env_var ("CLANG") ; let mut cmd = Command :: new (clang) ; cmd . arg ("-L") . arg (cwd ()) ; Self { cmd } } # [doc = " Provide an input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } # [doc = " Specify the name of the executable. The executable will be placed under the current directory"] # [doc = " and the extension will be determined by [`bin_name`]."] pub fn out_exe (& mut self , name : & str) -> & mut Self { self . cmd . arg ("-o") ; self . cmd . arg (bin_name (name)) ; self } # [doc = " Specify which target triple clang should target."] pub fn target (& mut self , target_triple : & str) -> & mut Self { self . cmd . arg ("-target") ; self . cmd . arg (target_triple) ; self } # [doc = " Pass `-nostdlib` to disable linking the C standard library."] pub fn no_stdlib (& mut self) -> & mut Self { self . cmd . arg ("-nostdlib") ; self } # [doc = " Specify architecture."] pub fn arch (& mut self , arch : & str) -> & mut Self { self . cmd . arg (format ! ("-march={arch}")) ; self } # [doc = " Specify LTO settings."] pub fn lto (& mut self , lto : & str) -> & mut Self { self . cmd . arg (format ! ("-flto={lto}")) ; self } # [doc = " Specify which ld to use."] pub fn use_ld (& mut self , ld : & str) -> & mut Self { self . cmd . arg (format ! ("-fuse-ld={ld}")) ; self } }
};
}
