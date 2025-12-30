// Generated macro for AttributeSeq (type)
macro_rules! Depcrate_requestAttributeSeq {
() => {
// Module: crate::request
// Provides: {"AttributeSeq"}
// Dependencies: {}
# [doc = " AttributeSeq corresponds to the type that is inlined in the CertReqMsg definition for the regInfo"] # [doc = " field, as shown below:"] # [doc = " ```text"] # [doc = "       regInfo   SEQUENCE SIZE(1..MAX) OF"] # [doc = "           SingleAttribute{{RegInfoSet}} OPTIONAL }"] # [doc = " ```"] pub type AttributeSeq = Vec < Attribute > ;
};
}
