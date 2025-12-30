// Generated macro for Tag (enum)
macro_rules! Depcrate_tagTag {
() => {
// Module: crate::tag
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " Tag \"IR\" type."] # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] pub (crate) enum Tag { # [doc = " Universal tags with an associated [`Asn1Type`]."] Universal (Asn1Type) , # [doc = " `APPLICATION` tags with an associated [`TagNumber`]."] Application { # [doc = " Is the inner ASN.1 type constructed?"] constructed : bool , # [doc = " Context-specific tag number"] number : TagNumber , } , # [doc = " `CONTEXT-SPECIFIC` tags with an associated [`TagNumber`]."] ContextSpecific { # [doc = " Is the inner ASN.1 type constructed?"] constructed : bool , # [doc = " Context-specific tag number"] number : TagNumber , } , # [doc = " `PRIVATE` tags with an associated [`TagNumber`]."] Private { # [doc = " Is the inner ASN.1 type constructed?"] constructed : bool , # [doc = " Context-specific tag number"] number : TagNumber , } , }
};
}
