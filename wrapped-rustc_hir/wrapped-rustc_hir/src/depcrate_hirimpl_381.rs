// Generated macro for impl_381 (impl)
macro_rules! Depcrate_hirimpl_381 {
() => {
// Module: crate::hir
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'hir > FnDecl < 'hir > { pub fn opt_delegation_sig_id (& self) -> Option < DefId > { if let FnRetTy :: Return (ty) = self . output && let TyKind :: InferDelegation (sig_id , _) = ty . kind { return Some (sig_id) ; } None } }
};
}
