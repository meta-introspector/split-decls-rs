macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T > PartialEq for IdxRange < T > { fn eq (& self , other : & Self) -> bool { self . range == other . range } }
    };
}

impl_25!()