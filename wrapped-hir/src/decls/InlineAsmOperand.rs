macro_rules! InlineAsmOperand {
    () => {
        # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub struct InlineAsmOperand { owner : DefWithBodyId , expr : ExprId , index : usize , }
    };
}

InlineAsmOperand!();