// Generated macro for codegen_global_asm_inner (function)
macro_rules! Depcrate_global_asmcodegen_global_asm_inner {
() => {
// Module: crate::global_asm
// Provides: {"codegen_global_asm_inner"}
// Dependencies: {}
fn codegen_global_asm_inner < 'tcx > (tcx : TyCtxt < 'tcx > , global_asm : & mut String , template : & [InlineAsmTemplatePiece] , operands : & [GlobalAsmOperandRef < 'tcx >] , options : InlineAsmOptions ,) { let is_x86 = matches ! (tcx . sess . asm_arch . unwrap () , InlineAsmArch :: X86 | InlineAsmArch :: X86_64) ; if is_x86 { if ! options . contains (InlineAsmOptions :: ATT_SYNTAX) { global_asm . push_str ("\n.intel_syntax noprefix\n") ; } else { global_asm . push_str ("\n.att_syntax\n") ; } } for piece in template { match * piece { InlineAsmTemplatePiece :: String (ref s) => global_asm . push_str (s) , InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier : _ , span } => { match operands [operand_idx] { GlobalAsmOperandRef :: Const { ref string } => { global_asm . push_str (string) ; } GlobalAsmOperandRef :: SymFn { instance } => { if cfg ! (not (feature = "inline_asm_sym")) { tcx . dcx () . span_err (span , "asm! and global_asm! sym operands are not yet supported" ,) ; } let symbol = tcx . symbol_name (instance) ; global_asm . push_str (symbol . name) ; } GlobalAsmOperandRef :: SymStatic { def_id } => { if cfg ! (not (feature = "inline_asm_sym")) { tcx . dcx () . span_err (span , "asm! and global_asm! sym operands are not yet supported" ,) ; } let instance = Instance :: mono (tcx , def_id) ; let symbol = tcx . symbol_name (instance) ; global_asm . push_str (symbol . name) ; } } } } } global_asm . push ('\n') ; if is_x86 { global_asm . push_str (".att_syntax\n\n") ; } }
};
}
