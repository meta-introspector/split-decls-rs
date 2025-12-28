macro_rules! deps {
    () => {
        Expr!();
        QPath!();
        Block!();
        ConstBlock!();
    };
}

macro_rules! InlineAsmOperand {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum InlineAsmOperand < 'hir > { In { reg : InlineAsmRegOrRegClass , expr : & 'hir Expr < 'hir > , } , Out { reg : InlineAsmRegOrRegClass , late : bool , expr : Option < & 'hir Expr < 'hir > > , } , InOut { reg : InlineAsmRegOrRegClass , late : bool , expr : & 'hir Expr < 'hir > , } , SplitInOut { reg : InlineAsmRegOrRegClass , late : bool , in_expr : & 'hir Expr < 'hir > , out_expr : Option < & 'hir Expr < 'hir > > , } , Const { anon_const : ConstBlock , } , SymFn { expr : & 'hir Expr < 'hir > , } , SymStatic { path : QPath < 'hir > , def_id : DefId , } , Label { block : & 'hir Block < 'hir > , } , }
    };
}

InlineAsmOperand!();