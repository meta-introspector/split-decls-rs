macro_rules! impl_407 {
    () => {
        impl < St : Stream > FusedFuture for Peek < '_ , St > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_407!();