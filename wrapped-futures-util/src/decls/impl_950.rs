macro_rules! impl_950 {
    () => {
        impl < S : FusedStream , F > FusedStream for SinkMapErr < S , F > { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
    };
}

impl_950!();