macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_1146 {
    () => {
        deps!();
        impl < 'a , R : AsyncRead + ? Sized + Unpin > Read < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut [u8]) -> Self { Self { reader , buf } } }
    };
}

impl_1146!()