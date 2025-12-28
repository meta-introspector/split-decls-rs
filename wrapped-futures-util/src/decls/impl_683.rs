macro_rules! impl_683 {
    () => {
        impl < St : TryStream + FusedStream > FusedStream for TryReadyChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_683!();