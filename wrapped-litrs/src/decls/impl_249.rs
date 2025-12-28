macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < 'a > Buffer for & 'a str { # [doc (hidden)] fn cut (self , range : Range < usize >) -> Self { & self [range] } type Cow = Cow < 'a , str > ; # [doc (hidden)] fn into_cow (self) -> Self :: Cow { self . into () } type ByteCow = Cow < 'a , [u8] > ; # [doc (hidden)] fn into_byte_cow (self) -> Self :: ByteCow { self . as_bytes () . into () } }
    };
}

impl_249!();