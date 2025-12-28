macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < const CAP : usize > Eq for ArrayString < CAP > { }
    };
}

impl_14!();