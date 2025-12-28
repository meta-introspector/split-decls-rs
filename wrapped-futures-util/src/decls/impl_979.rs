macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_979 {
    () => {
        deps!();
        impl < S , Item , U , St , F > FusedStream for WithFlatMap < S , Item , U , St , F > where S : FusedStream + Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , S :: Error > > , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
    };
}

impl_979!();