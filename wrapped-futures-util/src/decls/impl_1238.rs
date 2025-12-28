macro_rules! deps {
    () => {
        WriteAll!();
    };
}

macro_rules! impl_1238 {
    () => {
        deps!();
        impl < 'a , W : AsyncWrite + ? Sized + Unpin > WriteAll < 'a , W > { pub (super) fn new (writer : & 'a mut W , buf : & 'a [u8]) -> Self { Self { writer , buf } } }
    };
}

impl_1238!();