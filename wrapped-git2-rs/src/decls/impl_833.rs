macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! impl_833 {
    () => {
        deps!();
        impl < 'a > PartialEq for TreeEntry < 'a > { fn eq (& self , other : & TreeEntry < 'a >) -> bool { self . cmp (other) == Ordering :: Equal } }
    };
}

impl_833!()