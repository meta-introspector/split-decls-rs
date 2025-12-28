macro_rules! deps {
    () => {
        WriteVectored!();
    };
}

macro_rules! impl_1233 {
    () => {
        deps!();
        impl < 'a , 'b , W : AsyncWrite + ? Sized + Unpin > WriteVectored < 'a , 'b , W > { pub (super) fn new (writer : & 'a mut W , bufs : & 'a [IoSlice < 'b >]) -> Self { Self { writer , bufs } } }
    };
}

impl_1233!();