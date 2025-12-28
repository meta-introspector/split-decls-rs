macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Ord for Span { fn cmp (& self , other : & Span) -> Ordering { (& self . start , & self . end) . cmp (& (& other . start , & other . end)) } }
    };
}

impl_54!()