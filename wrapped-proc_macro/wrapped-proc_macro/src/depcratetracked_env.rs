// Generated macro for tracked_env (module)
macro_rules! Depcratetracked_env {
() => {
// Module: crate
// Provides: {"tracked_env"}
// Dependencies: {}
# [doc = " Tracked access to environment variables."] # [unstable (feature = "proc_macro_tracked_env" , issue = "99515")] pub mod tracked_env { use std :: env :: { self , VarError } ; use std :: ffi :: OsStr ; # [doc = " Retrieve an environment variable and add it to build dependency info."] # [doc = " The build system executing the compiler will know that the variable was accessed during"] # [doc = " compilation, and will be able to rerun the build when the value of that variable changes."] # [doc = " Besides the dependency tracking this function should be equivalent to `env::var` from the"] # [doc = " standard library, except that the argument must be UTF-8."] # [unstable (feature = "proc_macro_tracked_env" , issue = "99515")] pub fn var < K : AsRef < OsStr > + AsRef < str > > (key : K) -> Result < String , VarError > { let key : & str = key . as_ref () ; let value = crate :: bridge :: client :: FreeFunctions :: injected_env_var (key) . map_or_else (| | env :: var (key) , Ok) ; crate :: bridge :: client :: FreeFunctions :: track_env_var (key , value . as_deref () . ok ()) ; value } }
};
}
