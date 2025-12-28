macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl PartialOrd for Span { fn partial_cmp (& self , other : & Span) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_55!();