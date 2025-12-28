macro_rules! deps {
    () => {
        Close!();
    };
}

macro_rules! impl_1092 {
    () => {
        deps!();
        impl < 'a , W : AsyncWrite + ? Sized + Unpin > Close < 'a , W > { pub (super) fn new (writer : & 'a mut W) -> Self { Self { writer } } }
    };
}

impl_1092!();