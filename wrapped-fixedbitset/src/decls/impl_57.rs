macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl PartialOrd for Block { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_57!()