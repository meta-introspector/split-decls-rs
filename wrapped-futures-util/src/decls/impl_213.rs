macro_rules! impl_213 {
    () => {
        impl < Fut1 : FusedFuture , Fut2 : FusedFuture > FusedFuture for Join < Fut1 , Fut2 > { fn is_terminated (& self) -> bool { self . fut1 . is_terminated () && self . fut2 . is_terminated () } }
    };
}

impl_213!();