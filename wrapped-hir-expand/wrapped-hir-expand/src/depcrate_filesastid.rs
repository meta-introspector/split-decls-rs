// Generated macro for AstId (type)
macro_rules! Depcrate_filesAstId {
() => {
// Module: crate::files
// Provides: {"AstId"}
// Dependencies: {}
# [doc = " `AstId` points to an AST node in any file."] # [doc = ""] # [doc = " It is stable across reparses, and can be used as salsa key/value."] pub type AstId < N > = crate :: InFile < FileAstId < N > > ;
};
}
