// Generated macro for verify (function)
macro_rules! Depcrateverify {
() => {
// Module: crate
// Provides: {"verify"}
// Dependencies: {}
pub fn verify < 'chain , B : CryptoOps > (leaf : & VerificationCertificate < 'chain , B > , intermediates : & [VerificationCertificate < 'chain , B >] , policy : & Policy < '_ , B > , store : & Store < 'chain , B > ,) -> ValidationResult < 'chain , Chain < 'chain , B > , B > { let builder = ChainBuilder :: new (intermediates , policy , store) ; let mut budget = Budget :: new () ; builder . build_chain (leaf , & mut budget) }
};
}
