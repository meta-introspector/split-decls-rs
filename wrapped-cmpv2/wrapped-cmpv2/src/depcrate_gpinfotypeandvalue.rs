// Generated macro for InfoTypeAndValue (struct)
macro_rules! Depcrate_gpInfoTypeAndValue {
() => {
// Module: crate::gp
// Provides: {"InfoTypeAndValue"}
// Dependencies: {}
# [doc = " The `InfoTypeAndValue` type is defined in [RFC 4210 Section 5.3.19]"] # [doc = ""] # [doc = " ```text"] # [doc = "  InfoTypeAndValue ::= SEQUENCE {"] # [doc = "      infoType    INFO-TYPE-AND-VALUE."] # [doc = "                      &id({SupportedInfoSet}),"] # [doc = "      infoValue   INFO-TYPE-AND-VALUE."] # [doc = "                      &Type({SupportedInfoSet}{@infoType}) }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.19]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.19"] # [derive (Clone , Debug , Eq , PartialEq , PartialOrd , Ord , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct InfoTypeAndValue { pub oid : AttributeType , pub value : Option < AttributeValue > , }
};
}
