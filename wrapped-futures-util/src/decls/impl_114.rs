macro_rules! impl_114 {
    () => {
        impl < Fut : TryFuture > Future for IntoFuture < Fut > { type Output = Result < Fut :: Ok , Fut :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . future . try_poll (cx) } }
    };
}

impl_114!()