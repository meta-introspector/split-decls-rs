macro_rules! deps {
    () => {
        BytesMut!();
        Bytes!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl Extend < Bytes > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = Bytes > , { for bytes in iter { self . extend_from_slice (& bytes) } } }
    };
}

impl_207!();