macro_rules! deps {
    () => {
        Program!();
        Section!();
    };
}

macro_rules! impl_784 {
    () => {
        deps!();
        impl Program { # [doc = " Create a new instance."] pub const fn new_program (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Program) } }
    };
}

impl_784!();