macro_rules! deps {
    () => {
        InlineAsmOperand!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'hir > InlineAsmOperand < 'hir > { pub fn reg (& self) -> Option < InlineAsmRegOrRegClass > { match * self { Self :: In { reg , .. } | Self :: Out { reg , .. } | Self :: InOut { reg , .. } | Self :: SplitInOut { reg , .. } => Some (reg) , Self :: Const { .. } | Self :: SymFn { .. } | Self :: SymStatic { .. } | Self :: Label { .. } => None , } } pub fn is_clobber (& self) -> bool { matches ! (self , InlineAsmOperand :: Out { reg : InlineAsmRegOrRegClass :: Reg (_) , late : _ , expr : None }) } }
    };
}

impl_282!()