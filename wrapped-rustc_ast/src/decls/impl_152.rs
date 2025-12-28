macro_rules! deps {
    () => {
        Const!();
        InlineAsmOperand!();
        Label!();
        InlineAsmRegOrRegClass!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl InlineAsmOperand { pub fn reg (& self) -> Option < & InlineAsmRegOrRegClass > { match self { Self :: In { reg , .. } | Self :: Out { reg , .. } | Self :: InOut { reg , .. } | Self :: SplitInOut { reg , .. } => Some (reg) , Self :: Const { .. } | Self :: Sym { .. } | Self :: Label { .. } => None , } } }
    };
}

impl_152!();