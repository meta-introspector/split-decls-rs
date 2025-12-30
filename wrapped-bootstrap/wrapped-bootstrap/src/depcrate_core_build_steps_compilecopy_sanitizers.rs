// Generated macro for copy_sanitizers (function)
macro_rules! Depcrate_core_build_steps_compilecopy_sanitizers {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"copy_sanitizers"}
// Dependencies: {}
# [doc = " Copies sanitizer runtime libraries into target libdir."] fn copy_sanitizers (builder : & Builder < '_ > , compiler : & Compiler , target : TargetSelection ,) -> Vec < PathBuf > { let runtimes : Vec < llvm :: SanitizerRuntime > = builder . ensure (llvm :: Sanitizers { target }) ; if builder . config . dry_run () { return Vec :: new () ; } let mut target_deps = Vec :: new () ; let libdir = builder . sysroot_target_libdir (* compiler , target) ; for runtime in & runtimes { let dst = libdir . join (& runtime . name) ; builder . copy_link (& runtime . path , & dst , FileType :: NativeLibrary) ; if target == "x86_64-apple-darwin" || target == "aarch64-apple-darwin" || target == "aarch64-apple-ios" || target == "aarch64-apple-ios-sim" || target == "x86_64-apple-ios" { apple_darwin_update_library_name (builder , & dst , & format ! ("@rpath/{}" , runtime . name)) ; apple_darwin_sign_file (builder , & dst) ; } target_deps . push (dst) ; } target_deps }
};
}
