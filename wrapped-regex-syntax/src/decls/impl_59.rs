macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl PartialOrd for Position { fn partial_cmp (& self , other : & Position) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_59!()