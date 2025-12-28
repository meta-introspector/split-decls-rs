macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl PartialEq for Collector { # [doc = " Checks if both handles point to the same collector."] fn eq (& self , rhs : & Self) -> bool { Arc :: ptr_eq (& self . global , & rhs . global) } }
    };
}

impl_73!()