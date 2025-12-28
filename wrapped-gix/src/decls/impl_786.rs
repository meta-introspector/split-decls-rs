macro_rules! deps {
    () => {
        Path!();
        Section!();
    };
}

macro_rules! impl_786 {
    () => {
        deps!();
        impl Path { # [doc = " Create a new instance."] pub const fn new_path (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: Path) } }
    };
}

impl_786!()