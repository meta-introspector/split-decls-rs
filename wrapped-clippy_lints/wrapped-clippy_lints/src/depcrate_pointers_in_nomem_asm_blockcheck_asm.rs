// Generated macro for check_asm (function)
macro_rules! Depcrate_pointers_in_nomem_asm_blockcheck_asm {
() => {
// Module: crate::pointers_in_nomem_asm_block
// Provides: {"check_asm"}
// Dependencies: {}
fn check_asm (cx : & LateContext < '_ > , asm : & InlineAsm < '_ >) { if ! asm . options . contains (InlineAsmOptions :: NOMEM) { return ; } let spans = asm . operands . iter () . filter (| (op , _span) | has_in_operand_pointer (cx , op)) . map (| (_op , span) | * span) . collect :: < Vec < Span > > () ; if spans . is_empty () { return ; } span_lint_and_then (cx , POINTERS_IN_NOMEM_ASM_BLOCK , spans , "passing pointers to nomem asm block" , additional_notes ,) ; }
};
}
