// Generated macro for default_paths_to_vendor (function)
macro_rules! Depcrate_core_build_steps_vendordefault_paths_to_vendor {
() => {
// Module: crate::core::build_steps::vendor
// Provides: {"default_paths_to_vendor"}
// Dependencies: {}
# [doc = " Returns the cargo workspaces to vendor for `x vendor` and dist tarballs."] # [doc = ""] # [doc = " Returns a `Vec` of `(path_to_manifest, submodules_required)` where"] # [doc = " `path_to_manifest` is the cargo workspace, and `submodules_required` is"] # [doc = " the set of submodules that must be available."] pub fn default_paths_to_vendor (builder : & Builder < '_ >) -> Vec < (PathBuf , Vec < & 'static str >) > { [("src/tools/cargo/Cargo.toml" , vec ! ["src/tools/cargo"]) , ("src/tools/clippy/clippy_test_deps/Cargo.toml" , vec ! []) , ("src/tools/rust-analyzer/Cargo.toml" , vec ! []) , ("compiler/rustc_codegen_cranelift/Cargo.toml" , vec ! []) , ("compiler/rustc_codegen_gcc/Cargo.toml" , vec ! []) , ("library/Cargo.toml" , vec ! []) , ("src/bootstrap/Cargo.toml" , vec ! []) , ("src/tools/rustbook/Cargo.toml" , SUBMODULES_FOR_RUSTBOOK . into ()) , ("src/tools/rustc-perf/Cargo.toml" , vec ! ["src/tools/rustc-perf"]) , ("src/tools/opt-dist/Cargo.toml" , vec ! []) , ("src/doc/book/packages/trpl/Cargo.toml" , vec ! []) ,] . into_iter () . map (| (path , submodules) | (builder . src . join (path) , submodules)) . collect () }
};
}
