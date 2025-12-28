macro_rules! deps {
    () => {
        Align!();
        AbiAlign!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Deref for AbiAlign { type Target = Align ; fn deref (& self) -> & Self :: Target { & self . abi } }
    };
}

impl_45!()