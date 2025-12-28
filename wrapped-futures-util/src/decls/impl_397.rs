macro_rules! deps {
    () => {
        SelectNextSome!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < St : ? Sized + FusedStream + Unpin > FusedFuture for SelectNextSome < '_ , St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_397!()