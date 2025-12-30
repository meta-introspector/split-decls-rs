// Generated macro for OtherEvidence (struct)
macro_rules! Depcrate_timestamped_dataOtherEvidence {
() => {
// Module: crate::timestamped_data
// Provides: {"OtherEvidence"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " OtherEvidence ::= SEQUENCE {"] # [doc = "     oeType               OBJECT IDENTIFIER,"] # [doc = "     oeValue              ANY DEFINED BY oeType }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OtherEvidence { pub oe_type : ObjectIdentifier , pub oe_value : Any , }
};
}
