macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < St : FusedStream > FusedStream for InterleavePending < St > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_89!();