macro_rules! deps {
    () => {
        Executable!();
        Section!();
    };
}

macro_rules! impl_785 {
    () => {
        deps!();
        impl Executable { # [doc = " Create a new instance."] pub const fn new_executable (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Executable) } }
    };
}

impl_785!()