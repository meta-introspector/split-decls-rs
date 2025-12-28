macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > PartialOrd < & 'a T > for BytesMut where BytesMut : PartialOrd < T > , { fn partial_cmp (& self , other : & & 'a T) -> Option < cmp :: Ordering > { self . partial_cmp (* other) } }
    };
}

impl_235!();