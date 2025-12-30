// Generated macro for ArchiveTimeStamp (struct)
macro_rules! Depcrate_timestamped_dataArchiveTimeStamp {
() => {
// Module: crate::timestamped_data
// Provides: {"ArchiveTimeStamp"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " ArchiveTimeStamp ::= SEQUENCE {"] # [doc = "     digestAlgorithm [0] AlgorithmIdentifier OPTIONAL,"] # [doc = "     attributes      [1] Attributes OPTIONAL,"] # [doc = "     reducedHashtree [2] SEQUENCE OF PartialHashtree OPTIONAL,"] # [doc = "     timeStamp       ContentInfo }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ArchiveTimeStamp { # [asn1 (context_specific = "0" , optional = "true")] digest_algorithm : Option < AlgorithmIdentifierOwned > , # [asn1 (context_specific = "1" , optional = "true")] attributes : Option < Attributes > , # [asn1 (context_specific = "2" , optional = "true")] reduced_hashtree : Option < Vec < PartialHashtree > > , time_stamp : ContentInfo , }
};
}
