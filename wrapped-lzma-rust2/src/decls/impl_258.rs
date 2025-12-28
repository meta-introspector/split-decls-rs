macro_rules! deps {
    () => {
        Result!();
        Read!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl Read for & [u8] { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { let length = self . len () . min (buf . len ()) ; let (left , right) = self . split_at (length) ; buf [.. length] . copy_from_slice (left) ; * self = right ; Ok (length) } }
    };
}

impl_258!()