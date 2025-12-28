macro_rules! deps {
    () => {
        Lzma2Reader!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < R > Lzma2Reader < R > { # [doc = " Unwraps the reader, returning the underlying reader."] pub fn into_inner (self) -> R { self . inner } # [doc = " Returns a reference to the inner reader."] pub fn inner (& self) -> & R { & self . inner } # [doc = " Returns a mutable reference to the inner reader."] pub fn inner_mut (& mut self) -> & mut R { & mut self . inner } }
    };
}

impl_79!()