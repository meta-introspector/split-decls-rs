macro_rules! deps {
    () => {
        Direction!();
        CompactDirection!();
    };
}

macro_rules! impl_884 {
    () => {
        deps!();
        impl PartialEq < Direction > for CompactDirection { fn eq (& self , rhs : & Direction) -> bool { (* self as usize) == (* rhs as usize) } }
    };
}

impl_884!()