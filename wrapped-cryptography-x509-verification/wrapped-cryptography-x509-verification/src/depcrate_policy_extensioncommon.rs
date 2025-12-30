// Generated macro for common (module)
macro_rules! Depcrate_policy_extensioncommon {
() => {
// Module: crate::policy::extension
// Provides: {"common"}
// Dependencies: {}
mod common { use cryptography_x509 :: common :: Asn1Read ; use cryptography_x509 :: extensions :: { Extension , SequenceOfAccessDescriptions } ; use crate :: ops :: { CryptoOps , VerificationCertificate } ; use crate :: policy :: { Policy , ValidationResult } ; pub (crate) fn authority_information_access < 'chain , B : CryptoOps > (_policy : & Policy < '_ , B > , _cert : & VerificationCertificate < 'chain , B > , extn : Option < & Extension < '_ > > ,) -> ValidationResult < 'chain , () , B > { if let Some (extn) = extn { let _ : SequenceOfAccessDescriptions < '_ , Asn1Read > = extn . value () ? ; } Ok (()) } }
};
}
