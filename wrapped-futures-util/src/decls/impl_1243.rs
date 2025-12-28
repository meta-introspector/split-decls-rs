macro_rules! deps {
    () => {
        WriteAllVectored!();
    };
}

macro_rules! impl_1243 {
    () => {
        deps!();
        impl < 'a , 'b , W : AsyncWrite + ? Sized + Unpin > WriteAllVectored < 'a , 'b , W > { pub (super) fn new (writer : & 'a mut W , mut bufs : & 'a mut [IoSlice < 'b >]) -> Self { IoSlice :: advance_slices (& mut bufs , 0) ; Self { writer , bufs } } }
    };
}

impl_1243!()