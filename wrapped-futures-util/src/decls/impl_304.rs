macro_rules! impl_304 {
    () => {
        impl < St : FusedStream > FusedFuture for Count < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_304!()