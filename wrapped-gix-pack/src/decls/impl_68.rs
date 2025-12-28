macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T > Item < T > { # [doc = " Get the children"] pub fn children (& self) -> & [u32] { & self . children } }
    };
}

impl_68!()