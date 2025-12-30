// Generated macro for impl_425 (impl)
macro_rules! Depcrate_core_build_steps_llvmimpl_425 {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"impl_425"}
// Dependencies: {}
impl LdFlags { fn push_all (& mut self , s : impl AsRef < OsStr >) { let s = s . as_ref () ; self . exe . push (" ") ; self . exe . push (s) ; self . shared . push (" ") ; self . shared . push (s) ; self . module . push (" ") ; self . module . push (s) ; } }
};
}
