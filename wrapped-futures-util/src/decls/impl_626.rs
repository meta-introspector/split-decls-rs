macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl < St , Si , Item , E > FusedFuture for TryForward < St , Si , Item > where Si : Sink < Item , Error = E > , St : TryStream < Ok = Item , Error = E > , { fn is_terminated (& self) -> bool { self . sink . is_none () } }
    };
}

impl_626!()