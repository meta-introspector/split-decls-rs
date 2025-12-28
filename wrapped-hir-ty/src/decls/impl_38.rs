macro_rules! deps {
    () => {
        FnAbi!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl PartialEq for FnAbi { fn eq (& self , _other : & Self) -> bool { true } }
    };
}

impl_38!()