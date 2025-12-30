// Generated macro for has_in_operand_pointer (function)
macro_rules! Depcrate_pointers_in_nomem_asm_blockhas_in_operand_pointer {
() => {
// Module: crate::pointers_in_nomem_asm_block
// Provides: {"has_in_operand_pointer"}
// Dependencies: {}
fn has_in_operand_pointer (cx : & LateContext < '_ > , asm_op : & InlineAsmOperand < '_ >) -> bool { let asm_in_expr = match asm_op { InlineAsmOperand :: SymStatic { .. } | InlineAsmOperand :: Out { .. } | InlineAsmOperand :: Const { .. } | InlineAsmOperand :: SymFn { .. } | InlineAsmOperand :: Label { .. } => return false , InlineAsmOperand :: SplitInOut { in_expr , .. } => in_expr , InlineAsmOperand :: In { expr , .. } | InlineAsmOperand :: InOut { expr , .. } => expr , } ; cx . typeck_results () . expr_ty (asm_in_expr) . is_any_ptr () }
};
}
