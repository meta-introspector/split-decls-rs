// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Artifacts { pub fn include_dir (& self) -> & Path { & self . include_dir } pub fn lib_dir (& self) -> & Path { & self . lib_dir } pub fn libs (& self) -> & [String] { & self . libs } pub fn print_cargo_metadata (& self) { println ! ("cargo:rustc-link-search=native={}" , self . lib_dir . display ()) ; for lib in self . libs . iter () { println ! ("cargo:rustc-link-lib=static={}" , lib) ; } println ! ("cargo:include={}" , self . include_dir . display ()) ; println ! ("cargo:lib={}" , self . lib_dir . display ()) ; if self . target . contains ("windows") { println ! ("cargo:rustc-link-lib=user32") ; println ! ("cargo:rustc-link-lib=crypt32") ; println ! ("cargo:rustc-link-lib=advapi32") ; } else if self . target == "wasm32-wasi" { println ! ("cargo:rustc-link-lib=wasi-emulated-signal") ; println ! ("cargo:rustc-link-lib=wasi-emulated-process-clocks") ; println ! ("cargo:rustc-link-lib=wasi-emulated-mman") ; println ! ("cargo:rustc-link-lib=wasi-emulated-getpid") ; } } }
};
}
