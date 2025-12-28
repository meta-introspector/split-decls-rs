macro_rules! impl_426 {
    () => {
        impl < St : FusedStream > FusedStream for Skip < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_426!()