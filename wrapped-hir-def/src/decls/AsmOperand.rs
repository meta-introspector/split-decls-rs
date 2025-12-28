macro_rules! deps {
    () => {
        InlineAsmRegOrRegClass!();
        Label!();
        ExprId!();
        Const!();
    };
}

macro_rules! AsmOperand {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub enum AsmOperand { In { reg : InlineAsmRegOrRegClass , expr : ExprId , } , Out { reg : InlineAsmRegOrRegClass , expr : Option < ExprId > , late : bool , } , InOut { reg : InlineAsmRegOrRegClass , expr : ExprId , late : bool , } , SplitInOut { reg : InlineAsmRegOrRegClass , in_expr : ExprId , out_expr : Option < ExprId > , late : bool , } , Label (ExprId) , Const (ExprId) , Sym (Path) , }
    };
}

AsmOperand!()