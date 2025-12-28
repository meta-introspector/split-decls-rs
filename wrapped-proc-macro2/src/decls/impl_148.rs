macro_rules! deps {
    () => {
        LineColumn!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl PartialOrd for LineColumn { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_148!()