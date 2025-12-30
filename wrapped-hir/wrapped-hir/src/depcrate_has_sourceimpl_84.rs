// Generated macro for impl_84 (impl)
macro_rules! Depcrate_has_sourceimpl_84 {
() => {
// Module: crate::has_source
// Provides: {"impl_84"}
// Dependencies: {}
impl HasSource for InlineAsmOperand { type Ast = ast :: AsmOperandNamed ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let source_map = db . body_with_source_map (self . owner) . 1 ; if let Ok (src) = source_map . expr_syntax (self . expr) { let root = src . file_syntax (db) ; return src . map (| ast | match ast . to_node (& root) { Either :: Left (ast :: Expr :: AsmExpr (asm)) => asm . asm_pieces () . filter_map (| it | match it { ast :: AsmPiece :: AsmOperandNamed (it) => Some (it) , _ => None , }) . nth (self . index) , _ => None , }) . transpose () ; } None } }
};
}
