macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T > Future for Pending < T > { type Output = T ; fn poll (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < T > { Poll :: Pending } }
    };
}

impl_155!()