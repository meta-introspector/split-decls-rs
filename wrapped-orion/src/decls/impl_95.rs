macro_rules! deps {
    () => {
        StreamTag!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl PartialEq < StreamTag > for StreamTag { fn eq (& self , other : & StreamTag) -> bool { (self . as_byte () . ct_eq (& other . as_byte ())) . into () } }
    };
}

impl_95!()