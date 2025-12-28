macro_rules! deps {
    () => {
        Const!();
        Expr!();
        InlineAsmSym!();
        Label!();
        Inline!();
        InlineAsmRegOrRegClass!();
        Walkable!();
        AnonConst!();
        Block!();
    };
}

macro_rules! InlineAsmOperand {
    () => {
        deps!();
        # [doc = " Inline assembly operand."] # [doc = ""] # [doc = " E.g., `out(\"eax\") result` as in `asm!(\"mov eax, 2\", out(\"eax\") result)`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum InlineAsmOperand { In { reg : InlineAsmRegOrRegClass , expr : Box < Expr > , } , Out { reg : InlineAsmRegOrRegClass , late : bool , expr : Option < Box < Expr > > , } , InOut { reg : InlineAsmRegOrRegClass , late : bool , expr : Box < Expr > , } , SplitInOut { reg : InlineAsmRegOrRegClass , late : bool , in_expr : Box < Expr > , out_expr : Option < Box < Expr > > , } , Const { anon_const : AnonConst , } , Sym { sym : InlineAsmSym , } , Label { block : Box < Block > , } , }
    };
}

InlineAsmOperand!();