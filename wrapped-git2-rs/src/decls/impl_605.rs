macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl < 'repo > PartialOrd for Reference < 'repo > { fn partial_cmp (& self , other : & Reference < 'repo >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_605!();