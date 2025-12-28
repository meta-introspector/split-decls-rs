macro_rules! impl_411 {
    () => {
        impl < St : Stream > FusedFuture for PeekMut < '_ , St > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_411!()