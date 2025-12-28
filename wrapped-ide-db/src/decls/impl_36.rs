macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < InlineAsmOperand > for Definition { fn from (value : InlineAsmOperand) -> Self { Definition :: InlineAsmOperand (value) } }
    };
}

impl_36!();