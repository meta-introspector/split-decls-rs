macro_rules! deps {
    () => {
        TryFuture!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < F , T , E > TryFuture for F where F : ? Sized + Future < Output = Result < T , E > > , { type Ok = T ; type Error = E ; # [inline] fn try_poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll (cx) } }
    };
}

impl_9!()