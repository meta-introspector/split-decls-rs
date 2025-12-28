macro_rules! deps {
    () => {
        LocalFutureObj!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T > Future for LocalFutureObj < '_ , T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { unsafe { Pin :: new_unchecked (& mut * self . future) . poll (cx) } } }
    };
}

impl_37!();