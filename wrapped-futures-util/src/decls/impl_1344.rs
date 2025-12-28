macro_rules! deps {
    () => {
        Aborted!();
    };
}

macro_rules! impl_1344 {
    () => {
        deps!();
        impl < Fut > Future for Abortable < Fut > where Fut : Future , { type Output = Result < Fut :: Output , Aborted > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . try_poll (cx , | fut , cx | fut . poll (cx)) } }
    };
}

impl_1344!();