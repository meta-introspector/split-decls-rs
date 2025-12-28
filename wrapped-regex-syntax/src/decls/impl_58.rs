macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Ord for Position { fn cmp (& self , other : & Position) -> Ordering { self . offset . cmp (& other . offset) } }
    };
}

impl_58!()