macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_971 {
    () => {
        deps!();
        impl < S , Item , U , Fut , F > FusedStream for With < S , Item , U , Fut , F > where S : FusedStream + Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
    };
}

impl_971!();