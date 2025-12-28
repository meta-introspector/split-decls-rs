macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_771 {
    () => {
        deps!();
        impl PartialEq < & str > for ID { fn eq (& self , other : & & str) -> bool { self . 0 . as_str () == * other } }
    };
}

impl_771!();