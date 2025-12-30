// Generated macro for impl_211 (impl)
macro_rules! Depcrate_asm_syntaximpl_211 {
() => {
// Module: crate::asm_syntax
// Provides: {"impl_211"}
// Dependencies: {}
impl EarlyLintPass for InlineAsmX86IntelSyntax { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: InlineAsm (inline_asm) = & expr . kind { check_asm_syntax (INLINE_ASM_X86_INTEL_SYNTAX , cx , inline_asm , expr . span , AsmStyle :: Intel) ; } } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: GlobalAsm (inline_asm) = & item . kind { check_asm_syntax (INLINE_ASM_X86_INTEL_SYNTAX , cx , inline_asm , item . span , AsmStyle :: Intel) ; } } }
};
}
