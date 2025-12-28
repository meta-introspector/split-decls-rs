macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl PartialEq for Bytes { fn eq (& self , other : & Bytes) -> bool { self . as_slice () == other . as_slice () } }
    };
}

impl_80!()