macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_944 {
    () => {
        deps!();
        impl < S , Item , E > FusedStream for SinkErrInto < S , Item , E > where S : Sink < Item > + FusedStream , S :: Error : Into < E > , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
    };
}

impl_944!()