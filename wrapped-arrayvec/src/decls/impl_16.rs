macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < const CAP : usize > Borrow < str > for ArrayString < CAP > { fn borrow (& self) -> & str { self } }
    };
}

impl_16!()