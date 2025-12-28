macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl PartialOrd for File { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_439!();