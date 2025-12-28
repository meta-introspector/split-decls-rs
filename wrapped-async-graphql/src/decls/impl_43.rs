macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl PartialEq for Error { fn eq (& self , other : & Self) -> bool { self . message . eq (& other . message) && self . extensions . eq (& other . extensions) } }
    };
}

impl_43!()