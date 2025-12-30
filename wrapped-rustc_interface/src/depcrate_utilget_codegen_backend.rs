// Generated macro for get_codegen_backend (function)
macro_rules! Depcrate_utilget_codegen_backend {
() => {
// Module: crate::util
// Provides: {"get_codegen_backend"}
// Dependencies: {}
# [doc = " Get the codegen backend based on the name and specified sysroot."] # [doc = ""] # [doc = " A name of `None` indicates that the default backend should be used."] pub fn get_codegen_backend (early_dcx : & EarlyDiagCtxt , sysroot : & Sysroot , backend_name : Option < & str > , target : & Target ,) -> Box < dyn CodegenBackend > { static LOAD : OnceLock < unsafe fn () -> Box < dyn CodegenBackend > > = OnceLock :: new () ; let load = LOAD . get_or_init (| | { let backend = backend_name . or (target . default_codegen_backend . as_deref ()) . or (option_env ! ("CFG_DEFAULT_CODEGEN_BACKEND")) . unwrap_or ("llvm") ; match backend { filename if filename . contains ('.') => { load_backend_from_dylib (early_dcx , filename . as_ref ()) } # [cfg (feature = "llvm")] "llvm" => rustc_codegen_llvm :: LlvmCodegenBackend :: new , backend_name => get_codegen_sysroot (early_dcx , sysroot , backend_name) , } }) ; unsafe { load () } }
};
}
