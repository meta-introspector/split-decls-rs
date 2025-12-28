macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < A , B > FusedFuture for Select < A , B > where A : Future + Unpin , B : Future + Unpin , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_229!()