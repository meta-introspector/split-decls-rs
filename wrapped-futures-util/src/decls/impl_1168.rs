macro_rules! deps {
    () => {
        ReadToEnd!();
    };
}

macro_rules! impl_1168 {
    () => {
        deps!();
        impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadToEnd < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut Vec < u8 >) -> Self { let start_len = buf . len () ; Self { reader , buf , start_len } } }
    };
}

impl_1168!();