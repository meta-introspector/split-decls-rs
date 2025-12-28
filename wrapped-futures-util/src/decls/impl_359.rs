macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < St , Si , Item , E > FusedFuture for Forward < St , Si , Item > where Si : Sink < Item , Error = E > , St : Stream < Item = Item > , { fn is_terminated (& self) -> bool { self . sink . is_none () } }
    };
}

impl_359!();