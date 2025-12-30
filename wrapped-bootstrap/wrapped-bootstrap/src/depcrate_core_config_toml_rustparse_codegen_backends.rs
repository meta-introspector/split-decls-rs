// Generated macro for parse_codegen_backends (function)
macro_rules! Depcrate_core_config_toml_rustparse_codegen_backends {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"parse_codegen_backends"}
// Dependencies: {}
pub (crate) fn parse_codegen_backends (backends : Vec < String > , section : & str ,) -> Vec < CodegenBackendKind > { const CODEGEN_BACKEND_PREFIX : & str = "rustc_codegen_" ; let mut found_backends = vec ! [] ; for backend in & backends { if let Some (stripped) = backend . strip_prefix (CODEGEN_BACKEND_PREFIX) { panic ! ("Invalid value '{backend}' for '{section}.codegen-backends'. \
                Codegen backends are defined without the '{CODEGEN_BACKEND_PREFIX}' prefix. \
                Please, use '{stripped}' instead.") } if ! BUILTIN_CODEGEN_BACKENDS . contains (& backend . as_str ()) { println ! ("HELP: '{backend}' for '{section}.codegen-backends' might fail. \
                List of known codegen backends: {BUILTIN_CODEGEN_BACKENDS:?}") ; } let backend = match backend . as_str () { "llvm" => CodegenBackendKind :: Llvm , "cranelift" => CodegenBackendKind :: Cranelift , "gcc" => CodegenBackendKind :: Gcc , backend => CodegenBackendKind :: Custom (backend . to_string ()) , } ; found_backends . push (backend) ; } if found_backends . is_empty () { eprintln ! ("ERROR: `{section}.codegen-backends` should not be set to `[]`") ; exit ! (1) ; } found_backends }
};
}
