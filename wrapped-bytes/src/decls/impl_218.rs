macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl PartialEq < [u8] > for BytesMut { fn eq (& self , other : & [u8]) -> bool { & * * self == other } }
    };
}

impl_218!();