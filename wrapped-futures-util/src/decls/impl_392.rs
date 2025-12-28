macro_rules! deps {
    () => {
        Next!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl < St : ? Sized + FusedStream + Unpin > FusedFuture for Next < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_392!()