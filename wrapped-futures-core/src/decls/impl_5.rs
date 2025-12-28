macro_rules! deps {
    () => {
        FusedFuture!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < F : FusedFuture + ? Sized + Unpin > FusedFuture for & mut F { fn is_terminated (& self) -> bool { < F as FusedFuture > :: is_terminated (& * * self) } }
    };
}

impl_5!()