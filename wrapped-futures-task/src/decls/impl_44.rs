macro_rules! deps {
    () => {
        FutureObj!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > Future for FutureObj < '_ , T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { Pin :: new (& mut self . 0) . poll (cx) } }
    };
}

impl_44!()