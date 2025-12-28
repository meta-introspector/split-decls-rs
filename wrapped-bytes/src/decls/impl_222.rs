macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl PartialEq < str > for BytesMut { fn eq (& self , other : & str) -> bool { & * * self == other . as_bytes () } }
    };
}

impl_222!()