// Generated macro for add_sanitizer_libraries (function)
macro_rules! Depcrate_back_linkadd_sanitizer_libraries {
() => {
// Module: crate::back::link
// Provides: {"add_sanitizer_libraries"}
// Dependencies: {}
fn add_sanitizer_libraries (sess : & Session , flavor : LinkerFlavor , crate_type : CrateType , linker : & mut dyn Linker ,) { if sess . target . is_like_android { return ; } if sess . opts . unstable_opts . external_clangrt { return ; } if matches ! (crate_type , CrateType :: Rlib | CrateType :: Staticlib) { return ; } if matches ! (crate_type , CrateType :: Dylib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib) && ! (sess . target . is_like_darwin || sess . target . is_like_msvc) { return ; } let sanitizer = sess . opts . unstable_opts . sanitizer ; if sanitizer . contains (SanitizerSet :: ADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "asan") ; } if sanitizer . contains (SanitizerSet :: DATAFLOW) { link_sanitizer_runtime (sess , flavor , linker , "dfsan") ; } if sanitizer . contains (SanitizerSet :: LEAK) && ! sanitizer . contains (SanitizerSet :: ADDRESS) && ! sanitizer . contains (SanitizerSet :: HWADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "lsan") ; } if sanitizer . contains (SanitizerSet :: MEMORY) { link_sanitizer_runtime (sess , flavor , linker , "msan") ; } if sanitizer . contains (SanitizerSet :: THREAD) { link_sanitizer_runtime (sess , flavor , linker , "tsan") ; } if sanitizer . contains (SanitizerSet :: HWADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "hwasan") ; } if sanitizer . contains (SanitizerSet :: SAFESTACK) { link_sanitizer_runtime (sess , flavor , linker , "safestack") ; } }
};
}
