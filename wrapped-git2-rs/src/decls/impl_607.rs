macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl < 'repo > PartialEq for Reference < 'repo > { fn eq (& self , other : & Reference < 'repo >) -> bool { self . cmp (other) == Ordering :: Equal } }
    };
}

impl_607!()