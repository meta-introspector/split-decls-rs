macro_rules! deps {
    () => {
        TrySelect!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < A : Unpin , B : Unpin > Unpin for TrySelect < A , B > { }
    };
}

impl_253!()