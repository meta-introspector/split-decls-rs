macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl PartialEq for Interface { fn eq (& self , other : & Self) -> bool { self . def == other . def } }
    };
}

impl_302!()