macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Into < String > for ID < '_ > { fn into (self) -> String { self . 0 . to_string () } }
    };
}

impl_86!()