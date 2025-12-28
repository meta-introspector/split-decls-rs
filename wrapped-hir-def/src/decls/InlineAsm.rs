macro_rules! deps {
    () => {
        InlineAsmKind!();
        AsmOptions!();
        AsmOperand!();
    };
}

macro_rules! InlineAsm {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct InlineAsm { pub operands : Box < [(Option < Name > , AsmOperand)] > , pub options : AsmOptions , pub kind : InlineAsmKind , }
    };
}

InlineAsm!();