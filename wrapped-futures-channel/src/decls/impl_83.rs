macro_rules! deps {
    () => {
        Recv!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < St : ? Sized + FusedStream + Unpin > FusedFuture for Recv < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_83!()