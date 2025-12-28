macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T > Future for Ready < T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { Poll :: Ready (self . 0 . take () . expect ("Ready polled after completion")) } }
    };
}

impl_195!()