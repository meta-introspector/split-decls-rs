macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > PartialEq < & 'a T > for BytesMut where BytesMut : PartialEq < T > , { fn eq (& self , other : & & 'a T) -> bool { * self == * * other } }
    };
}

impl_234!();