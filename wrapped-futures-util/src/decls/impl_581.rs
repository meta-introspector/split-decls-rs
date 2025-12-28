macro_rules! impl_581 {
    () => {
        impl < St : FusedStream + UnwindSafe > FusedStream for CatchUnwind < St > { fn is_terminated (& self) -> bool { self . caught_unwind || self . stream . is_terminated () } }
    };
}

impl_581!();