macro_rules! impl_440 {
    () => {
        impl < St > FusedStream for Take < St > where St : FusedStream , { fn is_terminated (& self) -> bool { self . remaining == 0 || self . stream . is_terminated () } }
    };
}

impl_440!();