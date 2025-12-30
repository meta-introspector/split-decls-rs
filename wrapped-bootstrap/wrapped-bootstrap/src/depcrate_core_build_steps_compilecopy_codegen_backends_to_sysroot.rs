// Generated macro for copy_codegen_backends_to_sysroot (function)
macro_rules! Depcrate_core_build_steps_compilecopy_codegen_backends_to_sysroot {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"copy_codegen_backends_to_sysroot"}
// Dependencies: {}
# [doc = " Creates the `codegen-backends` folder for a compiler that's about to be"] # [doc = " assembled as a complete compiler."] # [doc = ""] # [doc = " This will take the codegen artifacts recorded in the given `stamp` and link them"] # [doc = " into an appropriate location for `target_compiler` to be a functional"] # [doc = " compiler."] fn copy_codegen_backends_to_sysroot (builder : & Builder < '_ > , stamp : BuildStamp , target_compiler : Compiler ,) { let dst = builder . sysroot_codegen_backends (target_compiler) ; t ! (fs :: create_dir_all (& dst) , dst) ; if builder . config . dry_run () { return ; } if stamp . path () . exists () { let file = get_codegen_backend_file (& stamp) ; builder . copy_link (& file , & dst . join (normalize_codegen_backend_name (builder , & file)) , FileType :: NativeLibrary ,) ; } }
};
}
