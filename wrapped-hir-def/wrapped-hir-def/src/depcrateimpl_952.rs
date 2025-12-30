// Generated macro for impl_952 (impl)
macro_rules! Depcrateimpl_952 {
() => {
// Module: crate
// Provides: {"impl_952"}
// Dependencies: {}
impl < T : AstIdNode > AstIdWithPath < T > { fn new (file_id : HirFileId , ast_id : FileAstId < T > , path : Interned < ModPath >) -> AstIdWithPath < T > { AstIdWithPath { ast_id : AstId :: new (file_id , ast_id) , path } } }
};
}
