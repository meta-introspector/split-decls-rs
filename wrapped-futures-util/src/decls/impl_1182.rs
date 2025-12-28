macro_rules! deps {
    () => {
        ReadUntil!();
    };
}

macro_rules! impl_1182 {
    () => {
        deps!();
        impl < 'a , R : AsyncBufRead + ? Sized + Unpin > ReadUntil < 'a , R > { pub (super) fn new (reader : & 'a mut R , byte : u8 , buf : & 'a mut Vec < u8 >) -> Self { Self { reader , byte , buf , read : 0 } } }
    };
}

impl_1182!();