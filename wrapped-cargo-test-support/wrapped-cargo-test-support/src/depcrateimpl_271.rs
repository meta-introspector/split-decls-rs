// Generated macro for impl_271 (impl)
macro_rules! Depcrateimpl_271 {
() => {
// Module: crate
// Provides: {"impl_271"}
// Dependencies: {}
impl TestEnvCommandExt for & mut ProcessBuilder { fn current_dir < S : AsRef < std :: path :: Path > > (self , path : S) -> Self { let path = path . as_ref () ; self . cwd (path) } fn env < S : AsRef < std :: ffi :: OsStr > > (self , key : & str , value : S) -> Self { self . env (key , value) } fn env_remove (self , key : & str) -> Self { self . env_remove (key) } }
};
}
