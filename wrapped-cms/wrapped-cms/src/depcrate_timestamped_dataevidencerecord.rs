// Generated macro for EvidenceRecord (struct)
macro_rules! Depcrate_timestamped_dataEvidenceRecord {
() => {
// Module: crate::timestamped_data
// Provides: {"EvidenceRecord"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " EvidenceRecord ::= SEQUENCE {"] # [doc = "     version                   INTEGER { v1(1) } ,"] # [doc = "     digestAlgorithms          SEQUENCE OF AlgorithmIdentifier,"] # [doc = "     cryptoInfos               [0] CryptoInfos OPTIONAL,"] # [doc = "     encryptionInfo            [1] EncryptionInfo OPTIONAL,"] # [doc = "     archiveTimeStampSequence  ArchiveTimeStampSequence"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EvidenceRecord { pub version : TsdVersion , pub digest_algorithm : Vec < AlgorithmIdentifierOwned > , # [asn1 (context_specific = "0" , optional = "true")] pub crypto_infos : Option < CryptoInfos > , # [asn1 (context_specific = "1" , optional = "true")] pub encryption_info : Option < EncryptionInfo > , pub archive_timestamp_sequence : ArchiveTimeStampSequence , }
};
}
