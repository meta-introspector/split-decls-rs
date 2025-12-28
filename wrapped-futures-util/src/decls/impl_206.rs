macro_rules! deps {
    () => {
        Ready!();
        AlwaysReady!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < T , F : Fn () -> T > Future for AlwaysReady < T , F > { type Output = T ; # [inline] fn poll (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { Poll :: Ready (self . 0 ()) } }
    };
}

impl_206!();