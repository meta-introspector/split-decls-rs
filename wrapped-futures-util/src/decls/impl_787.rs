macro_rules! impl_787 {
    () => {
        impl < S : Stream > super :: FusedStream for PollImmediate < S > { fn is_terminated (& self) -> bool { self . stream . is_none () } }
    };
}

impl_787!();