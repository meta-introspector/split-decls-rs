macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < A : Unpin , B : Unpin > Unpin for Select < A , B > { }
    };
}

impl_226!()