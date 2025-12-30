// Generated macro for impl_72 (impl)
macro_rules! Depcrate_policyimpl_72 {
() => {
// Module: crate::policy
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , B : CryptoOps > Deref for Policy < 'a , B > { type Target = PolicyDefinition < 'a , B > ; fn deref (& self) -> & Self :: Target { self . definition } }
};
}
