// Generated macro for AttrStyle (enum)
macro_rules! Depcrate_astAttrStyle {
() => {
// Module: crate::ast
// Provides: {"AttrStyle"}
// Dependencies: {}
# [doc = " Distinguishes between `Attribute`s that decorate items and Attributes that"] # [doc = " are contained as statements within items. These two cases need to be"] # [doc = " distinguished for pretty-printing."] # [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , HashStable_Generic , Walkable)] pub enum AttrStyle { Outer , Inner , }
};
}
