macro_rules! deps {
    () => {
        InlineAsmOperand!();
    };
}

macro_rules! InlineAsm {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct InlineAsm < 'hir > { pub asm_macro : ast :: AsmMacro , pub template : & 'hir [InlineAsmTemplatePiece] , pub template_strs : & 'hir [(Symbol , Option < Symbol > , Span)] , pub operands : & 'hir [(InlineAsmOperand < 'hir > , Span)] , pub options : InlineAsmOptions , pub line_spans : & 'hir [Span] , }
    };
}

InlineAsm!();