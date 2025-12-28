macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < Either < PathResolution , InlineAsmOperand > > for Definition { fn from (value : Either < PathResolution , InlineAsmOperand >) -> Self { value . either (Definition :: from , Definition :: from) } }
    };
}

impl_37!();