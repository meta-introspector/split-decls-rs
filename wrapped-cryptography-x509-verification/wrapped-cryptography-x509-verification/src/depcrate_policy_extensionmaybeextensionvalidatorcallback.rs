// Generated macro for MaybeExtensionValidatorCallback (type)
macro_rules! Depcrate_policy_extensionMaybeExtensionValidatorCallback {
() => {
// Module: crate::policy::extension
// Provides: {"MaybeExtensionValidatorCallback"}
// Dependencies: {}
pub type MaybeExtensionValidatorCallback < 'cb , B > = Arc < dyn for < 'chain > Fn (& Policy < '_ , B > , & VerificationCertificate < 'chain , B > , Option < & Extension < '_ > > ,) -> ValidationResult < 'chain , () , B > + Send + Sync + 'cb , > ;
};
}
