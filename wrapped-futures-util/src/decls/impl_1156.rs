macro_rules! deps {
    () => {
        ReadExact!();
    };
}

macro_rules! impl_1156 {
    () => {
        deps!();
        impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadExact < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut [u8]) -> Self { Self { reader , buf } } }
    };
}

impl_1156!()