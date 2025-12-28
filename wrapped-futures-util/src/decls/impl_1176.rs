macro_rules! deps {
    () => {
        ReadToString!();
    };
}

macro_rules! impl_1176 {
    () => {
        deps!();
        impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadToString < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut String) -> Self { let start_len = buf . len () ; Self { reader , bytes : mem :: take (buf) . into_bytes () , buf , start_len } } }
    };
}

impl_1176!();