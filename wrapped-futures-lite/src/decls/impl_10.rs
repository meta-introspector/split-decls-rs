macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { let this = self . project () ; (this . f) (cx) } }
    };
}

impl_10!();