macro_rules! impl_672 {
    () => {
        impl < St : TryStream + FusedStream > FusedStream for TryChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . items . is_empty () } }
    };
}

impl_672!();