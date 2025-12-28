macro_rules! deps {
    () => {
        ServerError!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl PartialEq for ServerError { fn eq (& self , other : & Self) -> bool { self . message . eq (& other . message) && self . locations . eq (& other . locations) && self . path . eq (& other . path) && self . extensions . eq (& other . extensions) } }
    };
}

impl_30!();