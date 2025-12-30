// Generated macro for PresentExtensionValidatorCallback (type)
macro_rules! Depcrate_policy_extensionPresentExtensionValidatorCallback {
() => {
// Module: crate::policy::extension
// Provides: {"PresentExtensionValidatorCallback"}
// Dependencies: {}
pub type PresentExtensionValidatorCallback < 'cb , B > = Arc < dyn for < 'chain > Fn (& Policy < '_ , B > , & VerificationCertificate < 'chain , B > , & Extension < '_ > ,) -> ValidationResult < 'chain , () , B > + Send + Sync + 'cb , > ;
};
}
