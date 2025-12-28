macro_rules! deps {
    () => {
        String!();
        Section!();
    };
}

macro_rules! impl_783 {
    () => {
        deps!();
        impl String { # [doc = " Create a new instance."] pub const fn new_string (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: String) } }
    };
}

impl_783!()