// Generated macro for AstIdWithPath (struct)
macro_rules! DepcrateAstIdWithPath {
() => {
// Module: crate
// Provides: {"AstIdWithPath"}
// Dependencies: {}
# [doc = " Helper wrapper for `AstId` with `ModPath`"] # [derive (Clone , Debug , Eq , PartialEq)] struct AstIdWithPath < T : AstIdNode > { ast_id : AstId < T > , path : Interned < ModPath > , }
};
}
