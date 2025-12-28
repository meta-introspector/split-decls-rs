macro_rules! deps {
    () => {
        Lazy!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < F , R > FusedFuture for Lazy < F > where F : FnOnce (& mut Context < '_ >) -> R , { fn is_terminated (& self) -> bool { self . f . is_none () } }
    };
}

impl_149!();