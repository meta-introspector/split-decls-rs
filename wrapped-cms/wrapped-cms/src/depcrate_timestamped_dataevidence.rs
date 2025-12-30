// Generated macro for Evidence (enum)
macro_rules! Depcrate_timestamped_dataEvidence {
() => {
// Module: crate::timestamped_data
// Provides: {"Evidence"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " Evidence ::= CHOICE {"] # [doc = "     tstEvidence    [0] TimeStampTokenEvidence,   -- see RFC 3161"] # [doc = "     ersEvidence    [1] EvidenceRecord,           -- see RFC 4998"] # [doc = "     otherEvidence  [2] OtherEvidence"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum Evidence { # [asn1 (context_specific = "0")] TstEvidence (TimeStampTokenEvidence) , # [asn1 (context_specific = "1")] ErsEvidence (EvidenceRecord) , # [asn1 (context_specific = "2")] OtherEvidence (OtherEvidence) , }
};
}
