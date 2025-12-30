// Generated macro for impl_112 (impl)
macro_rules! Depcrate_external_deps_c_cxx_compiler_gccimpl_112 {
() => {
// Module: crate::external_deps::c_cxx_compiler::gcc
// Provides: {"impl_112"}
// Dependencies: {}
impl Gcc { # [doc = " Construct a `gcc` invocation. This assumes that *a* suitable `gcc` is available in the"] # [doc = " environment."] # [doc = ""] # [doc = " Note that this does **not** prepopulate the `gcc` invocation with `CC_DEFAULT_FLAGS`."] # [track_caller] pub fn new () -> Self { let cmd = Command :: new ("gcc") ; Self { cmd } } # [doc = " Specify path of the input file."] pub fn input < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg (path . as_ref ()) ; self } # [doc = " Adds directories to the list that the linker searches for libraries."] # [doc = " Equivalent to `-L`."] pub fn library_search_path < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg ("-L") ; self . cmd . arg (path . as_ref ()) ; self } # [doc = " Specify `-o`."] pub fn out_exe (& mut self , name : & str) -> & mut Self { self . cmd . arg ("-o") ; self . cmd . arg (name) ; self } # [doc = " Specify path of the output binary."] pub fn output < P : AsRef < Path > > (& mut self , path : P) -> & mut Self { self . cmd . arg ("-o") ; self . cmd . arg (path . as_ref ()) ; self } # [doc = " Optimize the output at `-O3`."] pub fn optimize (& mut self) -> & mut Self { self . cmd . arg ("-O3") ; self } }
};
}
