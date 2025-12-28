macro_rules! impl_489 {
    () => {
        impl < St : Stream > FusedStream for ReadyChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_489!();