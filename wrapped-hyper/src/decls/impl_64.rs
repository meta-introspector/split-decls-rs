macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { (self . as_mut () . f) (cx) } }
    };
}

impl_64!();