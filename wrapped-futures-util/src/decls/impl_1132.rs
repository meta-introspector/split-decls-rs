macro_rules! deps {
    () => {
        Flush!();
    };
}

macro_rules! impl_1132 {
    () => {
        deps!();
        impl < 'a , W : AsyncWrite + ? Sized + Unpin > Flush < 'a , W > { pub (super) fn new (writer : & 'a mut W) -> Self { Self { writer } } }
    };
}

impl_1132!();