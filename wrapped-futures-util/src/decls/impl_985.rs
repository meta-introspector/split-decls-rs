macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_985 {
    () => {
        deps!();
        impl < S , Item > FusedStream for Buffer < S , Item > where S : Sink < Item > + FusedStream , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
    };
}

impl_985!()