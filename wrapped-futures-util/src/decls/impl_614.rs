macro_rules! deps {
    () => {
        TryNext!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < St : ? Sized + TryStream + Unpin + FusedStream > FusedFuture for TryNext < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_614!();