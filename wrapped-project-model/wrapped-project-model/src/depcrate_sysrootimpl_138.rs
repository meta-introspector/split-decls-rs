// Generated macro for impl_138 (impl)
macro_rules! Depcrate_sysrootimpl_138 {
() => {
// Module: crate::sysroot
// Provides: {"impl_138"}
// Dependencies: {}
impl Sysroot { pub const fn empty () -> Sysroot { Sysroot { root : None , rust_lib_src_root : None , workspace : RustLibSrcWorkspace :: Empty , error : None , } } # [doc = " Returns sysroot \"root\" directory, where `bin/`, `etc/`, `lib/`, `libexec/`"] # [doc = " subfolder live, like:"] # [doc = " `$HOME/.rustup/toolchains/nightly-2022-07-23-x86_64-unknown-linux-gnu`"] pub fn root (& self) -> Option < & AbsPath > { self . root . as_deref () } # [doc = " Returns the sysroot \"source\" directory, where stdlib sources are located, like:"] # [doc = " `$HOME/.rustup/toolchains/nightly-2022-07-23-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library`"] pub fn rust_lib_src_root (& self) -> Option < & AbsPath > { self . rust_lib_src_root . as_deref () } pub fn is_rust_lib_src_empty (& self) -> bool { match & self . workspace { RustLibSrcWorkspace :: Workspace (ws) => ws . packages () . next () . is_none () , RustLibSrcWorkspace :: Json (project_json) => project_json . n_crates () == 0 , RustLibSrcWorkspace :: Stitched (stitched) => stitched . crates . is_empty () , RustLibSrcWorkspace :: Empty => true , } } pub fn error (& self) -> Option < & str > { self . error . as_deref () } pub fn num_packages (& self) -> usize { match & self . workspace { RustLibSrcWorkspace :: Workspace (ws) => ws . packages () . count () , RustLibSrcWorkspace :: Json (project_json) => project_json . n_crates () , RustLibSrcWorkspace :: Stitched (stitched) => stitched . crates . len () , RustLibSrcWorkspace :: Empty => 0 , } } pub (crate) fn workspace (& self) -> & RustLibSrcWorkspace { & self . workspace } }
};
}
