macro_rules! deps {
    () => {
        Receiver!();
        Canceled!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T > Future for Receiver < T > { type Output = Result < T , Canceled > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < T , Canceled > > { self . inner . recv (cx) } }
    };
}

impl_118!()