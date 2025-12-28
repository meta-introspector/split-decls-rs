macro_rules! impl_454 {
    () => {
        impl < St , Fut > FusedStream for TakeUntil < St , Fut > where St : Stream , Fut : Future , { fn is_terminated (& self) -> bool { self . is_stopped () } }
    };
}

impl_454!();