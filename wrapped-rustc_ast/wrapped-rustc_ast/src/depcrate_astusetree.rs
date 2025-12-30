// Generated macro for UseTree (struct)
macro_rules! Depcrate_astUseTree {
() => {
// Module: crate::ast
// Provides: {"UseTree"}
// Dependencies: {}
# [doc = " A tree of paths sharing common prefixes."] # [doc = " Used in `use` items both at top-level and inside of braces in import groups."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct UseTree { pub prefix : Path , pub kind : UseTreeKind , pub span : Span , }
};
}
