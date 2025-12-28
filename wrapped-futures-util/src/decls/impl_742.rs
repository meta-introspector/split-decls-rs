macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_742 {
    () => {
        deps!();
        impl < I > Iter < I > { # [doc = " Acquires a reference to the underlying iterator that this stream is pulling from."] pub fn get_ref (& self) -> & I { & self . iter } # [doc = " Acquires a mutable reference to the underlying iterator that this stream is pulling from."] pub fn get_mut (& mut self) -> & mut I { & mut self . iter } # [doc = " Consumes this stream, returning the underlying iterator."] pub fn into_inner (self) -> I { self . iter } }
    };
}

impl_742!()