macro_rules! deps {
    () => {
        AbiAlign!();
        Align!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Deref for AbiAlign { type Target = Align ; fn deref (& self) -> & Self :: Target { & self . abi } }
    };
}

impl_86!();