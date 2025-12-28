macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_1228 {
    () => {
        deps!();
        impl < 'a , W : AsyncWrite + ? Sized + Unpin > Write < 'a , W > { pub (super) fn new (writer : & 'a mut W , buf : & 'a [u8]) -> Self { Self { writer , buf } } }
    };
}

impl_1228!()