// Generated macro for impl_154 (impl)
macro_rules! Depcrate_cmacimpl_154 {
() => {
// Module: crate::cmac
// Provides: {"impl_154"}
// Dependencies: {}
impl Clone for LcPtr < CMAC_CTX > { fn clone (& self) -> Self { let mut new_ctx = LcPtr :: new (unsafe { CMAC_CTX_new () }) . expect ("CMAC_CTX_new failed") ; unsafe { assert ! (1 == CMAC_CTX_copy (* new_ctx . as_mut () , * self . as_const ()) , "CMAC_CTX_copy failed") ; } new_ctx } }
};
}
