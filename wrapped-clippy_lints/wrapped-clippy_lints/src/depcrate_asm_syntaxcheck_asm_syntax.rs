// Generated macro for check_asm_syntax (function)
macro_rules! Depcrate_asm_syntaxcheck_asm_syntax {
() => {
// Module: crate::asm_syntax
// Provides: {"check_asm_syntax"}
// Dependencies: {}
fn check_asm_syntax (lint : & 'static Lint , cx : & EarlyContext < '_ > , inline_asm : & InlineAsm , span : Span , check_for : AsmStyle ,) { if matches ! (cx . sess () . asm_arch , Some (InlineAsmArch :: X86 | InlineAsmArch :: X86_64)) { let style = if inline_asm . options . contains (InlineAsmOptions :: ATT_SYNTAX) { AsmStyle :: Att } else { AsmStyle :: Intel } ; if style == check_for { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , lint , span , format ! ("{style} x86 assembly syntax used") , | diag | { diag . help (format ! ("use {} x86 assembly syntax" , ! style)) ; }) ; } } }
};
}
