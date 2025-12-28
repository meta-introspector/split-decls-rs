macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl PartialEq for BytesMut { fn eq (& self , other : & BytesMut) -> bool { self . as_slice () == other . as_slice () } }
    };
}

impl_193!();