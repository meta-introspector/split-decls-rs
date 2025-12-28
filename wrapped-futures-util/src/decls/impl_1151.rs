macro_rules! deps {
    () => {
        ReadVectored!();
    };
}

macro_rules! impl_1151 {
    () => {
        deps!();
        impl < 'a , 'b , R : AsyncRead + ? Sized + Unpin > ReadVectored < 'a , 'b , R > { pub (super) fn new (reader : & 'a mut R , bufs : & 'a mut [IoSliceMut < 'b >]) -> Self { Self { reader , bufs } } }
    };
}

impl_1151!();