macro_rules! deps {
    () => {
        InlineAsm!();
        InlineAsmOperand!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl InlineAsm < '_ > { pub fn contains_label (& self) -> bool { self . operands . iter () . any (| x | matches ! (x . 0 , InlineAsmOperand :: Label { .. })) } }
    };
}

impl_284!()