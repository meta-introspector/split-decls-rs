macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl PartialEq for Error { fn eq (& self , other : & Self) -> bool { self . code == other . code } }
    };
}

impl_75!()