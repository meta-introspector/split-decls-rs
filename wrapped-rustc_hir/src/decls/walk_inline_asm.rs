macro_rules! deps {
    () => {
        InlineAsmOperand!();
        Visitor!();
        InlineAsm!();
    };
}

macro_rules! walk_inline_asm {
    () => {
        deps!();
        pub fn walk_inline_asm < 'v , V : Visitor < 'v > > (visitor : & mut V , asm : & 'v InlineAsm < 'v > , id : HirId ,) -> V :: Result { for (op , op_sp) in asm . operands { match op { InlineAsmOperand :: In { expr , .. } | InlineAsmOperand :: InOut { expr , .. } => { try_visit ! (visitor . visit_expr (expr)) ; } InlineAsmOperand :: Out { expr , .. } => { visit_opt ! (visitor , visit_expr , expr) ; } InlineAsmOperand :: SplitInOut { in_expr , out_expr , .. } => { try_visit ! (visitor . visit_expr (in_expr)) ; visit_opt ! (visitor , visit_expr , out_expr) ; } InlineAsmOperand :: Const { anon_const , .. } => { try_visit ! (visitor . visit_inline_const (anon_const)) ; } InlineAsmOperand :: SymFn { expr , .. } => { try_visit ! (visitor . visit_expr (expr)) ; } InlineAsmOperand :: SymStatic { path , .. } => { try_visit ! (visitor . visit_qpath (path , id , * op_sp)) ; } InlineAsmOperand :: Label { block } => try_visit ! (visitor . visit_block (block)) , } } V :: Result :: output () }
    };
}

walk_inline_asm!();