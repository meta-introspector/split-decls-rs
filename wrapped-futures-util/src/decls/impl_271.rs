macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < A , B > FusedFuture for Either < A , B > where A : FusedFuture , B : FusedFuture < Output = A :: Output > , { fn is_terminated (& self) -> bool { match self { Self :: Left (x) => x . is_terminated () , Self :: Right (x) => x . is_terminated () , } } }
    };
}

impl_271!()