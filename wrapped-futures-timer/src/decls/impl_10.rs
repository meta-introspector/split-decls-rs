macro_rules! deps {
    () => {
        Delay!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Future for Delay { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { Pin :: new (& mut * Pin :: into_inner (self) . 0) . poll (cx) } }
    };
}

impl_10!()