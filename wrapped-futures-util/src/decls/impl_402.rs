macro_rules! impl_402 {
    () => {
        impl < St : Stream > FusedStream for Peekable < St > { fn is_terminated (& self) -> bool { self . peeked . is_none () && self . stream . is_terminated () } }
    };
}

impl_402!()