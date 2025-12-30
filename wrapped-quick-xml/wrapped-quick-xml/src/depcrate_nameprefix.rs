// Generated macro for Prefix (struct)
macro_rules! Depcrate_namePrefix {
() => {
// Module: crate::name
// Provides: {"Prefix"}
// Dependencies: {}
# [doc = " A [namespace prefix] part of the [qualified name](QName) of an element tag"] # [doc = " or an attribute: a `prefix` in `<prefix:local-element-name>` or"] # [doc = " `prefix:local-attribute-name=\"attribute value\"`."] # [doc = ""] # [doc = " [namespace prefix]: https://www.w3.org/TR/xml-names11/#dt-prefix"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "serde-types" , derive (serde :: Deserialize , serde :: Serialize))] pub struct Prefix < 'a > (& 'a [u8]) ;
};
}
