macro_rules! deps {
    () => {
        Semantics!();
        IeeeFloat!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < S : Semantics > PartialEq for IeeeFloat < S > { fn eq (& self , rhs : & Self) -> bool { self . partial_cmp (rhs) == Some (Ordering :: Equal) } }
    };
}

impl_37!()