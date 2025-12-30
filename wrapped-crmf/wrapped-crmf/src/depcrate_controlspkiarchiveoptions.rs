// Generated macro for PkiArchiveOptions (enum)
macro_rules! Depcrate_controlsPkiArchiveOptions {
() => {
// Module: crate::controls
// Provides: {"PkiArchiveOptions"}
// Dependencies: {}
# [doc = " The `PKIArchiveOptions` control is defined in [RFC 4211 Section 6.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PKIArchiveOptions ::= CHOICE {"] # [doc = "       encryptedPrivKey     [0] EncryptedKey,"] # [doc = "       -- the actual value of the private key"] # [doc = "       keyGenParameters     [1] KeyGenParameters,"] # [doc = "       -- parameters that allow the private key to be re-generated"] # [doc = "       archiveRemGenPrivKey [2] BOOLEAN }"] # [doc = "       -- set to TRUE if sender wishes receiver to archive the private"] # [doc = "       -- key of a key pair that the receiver generates in response to"] # [doc = "       -- this request; set to FALSE if no archive is desired."] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6.4]: https://www.rfc-editor.org/rfc/rfc4211#section-6.4"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] # [allow (missing_docs)] pub enum PkiArchiveOptions { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true")] EncryptedPrivKey (EncryptedKey) , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true")] KeyGenParameters (KeyGenParameters) , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "false")] ArchiveRemGenPrivKey (bool) , }
};
}
