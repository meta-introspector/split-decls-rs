// Generated macro for PatField (struct)
macro_rules! Depcrate_astPatField {
() => {
// Module: crate::ast
// Provides: {"PatField"}
// Dependencies: {}
# [doc = " A single field in a struct pattern."] # [doc = ""] # [doc = " Patterns like the fields of `Foo { x, ref y, ref mut z }`"] # [doc = " are treated the same as `x: x, y: ref y, z: ref mut z`,"] # [doc = " except when `is_shorthand` is true."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct PatField { # [doc = " The identifier for the field."] pub ident : Ident , # [doc = " The pattern the field is destructured to."] pub pat : Box < Pat > , pub is_shorthand : bool , pub attrs : AttrVec , pub id : NodeId , pub span : Span , pub is_placeholder : bool , }
};
}
