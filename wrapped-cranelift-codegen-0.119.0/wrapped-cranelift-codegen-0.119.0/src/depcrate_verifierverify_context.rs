// Generated macro for verify_context (function)
macro_rules! Depcrate_verifierverify_context {
() => {
// Module: crate::verifier
// Provides: {"verify_context"}
// Dependencies: {}
# [doc = " Verify `func` after checking the integrity of associated context data structures `cfg` and"] # [doc = " `domtree`."] pub fn verify_context < 'a , FOI : Into < FlagsOrIsa < 'a > > > (func : & Function , cfg : & ControlFlowGraph , domtree : & DominatorTree , fisa : FOI , errors : & mut VerifierErrors ,) -> VerifierStepResult { let _tt = timing :: verifier () ; let verifier = Verifier :: new (func , fisa . into ()) ; if cfg . is_valid () { verifier . cfg_integrity (cfg , errors) ? ; } if domtree . is_valid () { verifier . domtree_integrity (domtree , errors) ? ; } verifier . run (errors) }
};
}
