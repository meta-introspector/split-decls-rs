macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! impl_831 {
    () => {
        deps!();
        impl < 'a > PartialOrd for TreeEntry < 'a > { fn partial_cmp (& self , other : & TreeEntry < 'a >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_831!();