// Generated macro for AttrKind (enum)
macro_rules! Depcrate_astAttrKind {
() => {
// Module: crate::ast
// Provides: {"AttrKind"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AttrKind { # [doc = " A normal attribute."] Normal (Box < NormalAttr >) , # [doc = " A doc comment (e.g. `/// ...`, `//! ...`, `/** ... */`, `/*! ... */`)."] # [doc = " Doc attributes (e.g. `#[doc=\"...\"]`) are represented with the `Normal`"] # [doc = " variant (which is much less compact and thus more expensive)."] DocComment (CommentKind , Symbol) , }
};
}
