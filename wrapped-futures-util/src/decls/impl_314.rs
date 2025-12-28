macro_rules! impl_314 {
    () => {
        impl < St : Stream + FusedStream > FusedStream for Enumerate < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_314!();