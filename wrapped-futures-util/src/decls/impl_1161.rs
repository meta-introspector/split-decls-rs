macro_rules! deps {
    () => {
        ReadLine!();
    };
}

macro_rules! impl_1161 {
    () => {
        deps!();
        impl < 'a , R : AsyncBufRead + ? Sized + Unpin > ReadLine < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut String) -> Self { Self { reader , bytes : mem :: take (buf) . into_bytes () , buf , read : 0 , finished : false } } }
    };
}

impl_1161!();