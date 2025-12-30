// Generated macro for impl_214 (impl)
macro_rules! Depcrate_asm_syntaximpl_214 {
() => {
// Module: crate::asm_syntax
// Provides: {"impl_214"}
// Dependencies: {}
impl EarlyLintPass for InlineAsmX86AttSyntax { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: InlineAsm (inline_asm) = & expr . kind { check_asm_syntax (INLINE_ASM_X86_ATT_SYNTAX , cx , inline_asm , expr . span , AsmStyle :: Att) ; } } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: GlobalAsm (inline_asm) = & item . kind { check_asm_syntax (INLINE_ASM_X86_ATT_SYNTAX , cx , inline_asm , item . span , AsmStyle :: Att) ; } } }
};
}
