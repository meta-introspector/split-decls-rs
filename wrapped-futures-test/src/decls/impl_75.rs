macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < St : FusedStream > FusedStream for AssertUnmoved < St > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_75!()