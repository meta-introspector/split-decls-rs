macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'a > Extend < & 'a u8 > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = & 'a u8 > , { self . extend (iter . into_iter () . copied ()) } }
    };
}

impl_206!()