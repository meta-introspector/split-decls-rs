// Generated macro for LocalName (struct)
macro_rules! Depcrate_nameLocalName {
() => {
// Module: crate::name
// Provides: {"LocalName"}
// Dependencies: {}
# [doc = " A [local (unqualified) name] of an element or an attribute, i.e. a name"] # [doc = " without [prefix](Prefix)."] # [doc = ""] # [doc = " [local (unqualified) name]: https://www.w3.org/TR/xml-names11/#dt-localname"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "serde-types" , derive (serde :: Deserialize , serde :: Serialize))] pub struct LocalName < 'a > (pub (crate) & 'a [u8]) ;
};
}
