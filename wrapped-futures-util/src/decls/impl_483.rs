macro_rules! impl_483 {
    () => {
        impl < St : FusedStream > FusedStream for Chunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . items . is_empty () } }
    };
}

impl_483!()