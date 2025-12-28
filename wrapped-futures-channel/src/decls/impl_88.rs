macro_rules! deps {
    () => {
        UnboundedReceiver!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T > FusedStream for UnboundedReceiver < T > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_88!()