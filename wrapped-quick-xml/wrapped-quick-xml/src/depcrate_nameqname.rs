// Generated macro for QName (struct)
macro_rules! Depcrate_nameQName {
() => {
// Module: crate::name
// Provides: {"QName"}
// Dependencies: {}
# [doc = " A [qualified name] of an element or an attribute, including an optional"] # [doc = " namespace [prefix](Prefix) and a [local name](LocalName)."] # [doc = ""] # [doc = " [qualified name]: https://www.w3.org/TR/xml-names11/#dt-qualname"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "serde-types" , derive (serde :: Deserialize , serde :: Serialize))] pub struct QName < 'a > (pub & 'a [u8]) ;
};
}
