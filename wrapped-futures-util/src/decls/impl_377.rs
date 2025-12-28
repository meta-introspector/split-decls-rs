macro_rules! deps {
    () => {
        StreamFuture!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < St : Stream + Unpin > FusedFuture for StreamFuture < St > { fn is_terminated (& self) -> bool { self . stream . is_none () } }
    };
}

impl_377!()