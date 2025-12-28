macro_rules! deps {
    () => {
        FnAbi!();
    };
}

macro_rules! impl_1023 {
    () => {
        deps!();
        impl PartialEq for FnAbi { fn eq (& self , _other : & Self) -> bool { true } }
    };
}

impl_1023!();