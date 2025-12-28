macro_rules! deps {
    () => {
        AwokenCount!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl PartialEq < usize > for AwokenCount { fn eq (& self , other : & usize) -> bool { self . get () == * other } }
    };
}

impl_37!();