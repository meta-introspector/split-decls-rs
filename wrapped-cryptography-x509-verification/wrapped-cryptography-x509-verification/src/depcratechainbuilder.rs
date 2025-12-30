// Generated macro for ChainBuilder (struct)
macro_rules! DepcrateChainBuilder {
() => {
// Module: crate
// Provides: {"ChainBuilder"}
// Dependencies: {}
struct ChainBuilder < 'a , 'chain , B : CryptoOps > { intermediates : & 'a [VerificationCertificate < 'chain , B >] , policy : & 'a Policy < 'a , B > , store : & 'a Store < 'chain , B > , }
};
}
