macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Extend < u8 > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = u8 > , { let iter = iter . into_iter () ; let (lower , _) = iter . size_hint () ; self . reserve (lower) ; for b in iter { self . put_u8 (b) ; } } }
    };
}

impl_205!()