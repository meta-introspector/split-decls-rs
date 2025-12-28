macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1084 {
    () => {
        deps!();
        impl < 'a > ByteString < 'a > { # [doc = " Acquire a mutable reference to the bytes."] # [doc = ""] # [doc = " Clones the bytes if they are shared."] pub fn to_mut (& mut self) -> & mut Vec < u8 > { self . 0 . to_mut () } # [doc = " Get the bytes as a slice."] pub fn as_slice (& self) -> & [u8] { self . 0 . as_ref () } }
    };
}

impl_1084!()