macro_rules! deps {
    () => {
        FusedStream!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < F : ? Sized + FusedStream + Unpin > FusedStream for & mut F { fn is_terminated (& self) -> bool { < F as FusedStream > :: is_terminated (& * * self) } }
    };
}

impl_18!();