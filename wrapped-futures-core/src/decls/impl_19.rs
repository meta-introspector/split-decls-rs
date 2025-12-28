macro_rules! deps {
    () => {
        FusedStream!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < P > FusedStream for Pin < P > where P : DerefMut + Unpin , P :: Target : FusedStream , { fn is_terminated (& self) -> bool { < P :: Target as FusedStream > :: is_terminated (& * * self) } }
    };
}

impl_19!()