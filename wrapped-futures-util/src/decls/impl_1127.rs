macro_rules! deps {
    () => {
        FillBuf!();
    };
}

macro_rules! impl_1127 {
    () => {
        deps!();
        impl < 'a , R : AsyncBufRead + ? Sized + Unpin > FillBuf < 'a , R > { pub (super) fn new (reader : & 'a mut R) -> Self { Self { reader : Some (reader) } } }
    };
}

impl_1127!();