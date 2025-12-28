macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl PartialEq < String > for BytesMut { fn eq (& self , other : & String) -> bool { * self == other [..] } }
    };
}

impl_230!();