macro_rules! deps {
    () => {
        FusedFuture!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < P > FusedFuture for Pin < P > where P : DerefMut + Unpin , P :: Target : FusedFuture , { fn is_terminated (& self) -> bool { < P :: Target as FusedFuture > :: is_terminated (& * * self) } }
    };
}

impl_6!();