macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < A , B > FusedStream for Either < A , B > where A : FusedStream , B : FusedStream < Item = A :: Item > , { fn is_terminated (& self) -> bool { match self { Self :: Left (x) => x . is_terminated () , Self :: Right (x) => x . is_terminated () , } } }
    };
}

impl_273!()