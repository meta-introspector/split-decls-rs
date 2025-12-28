macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl PartialOrd < FileTime > for Time { fn partial_cmp (& self , other : & FileTime) -> Option < Ordering > { self . partial_cmp (& Time :: from (* other)) } }
    };
}

impl_98!();