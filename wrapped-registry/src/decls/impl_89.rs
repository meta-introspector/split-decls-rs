macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl PartialEq for Data { fn eq (& self , other : & Self) -> bool { self . deref () == other . deref () } }
    };
}

impl_89!();