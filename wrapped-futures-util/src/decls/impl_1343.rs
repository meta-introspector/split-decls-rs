macro_rules! deps {
    () => {
        Aborted!();
        Pending!();
        Ready!();
    };
}

macro_rules! impl_1343 {
    () => {
        deps!();
        impl < T > Abortable < T > { fn try_poll < I > (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , poll : impl Fn (Pin < & mut T > , & mut Context < '_ >) -> Poll < I > ,) -> Poll < Result < I , Aborted > > { if self . is_aborted () { return Poll :: Ready (Err (Aborted)) ; } if let Poll :: Ready (x) = poll (self . as_mut () . project () . task , cx) { return Poll :: Ready (Ok (x)) ; } self . inner . waker . register (cx . waker ()) ; if self . is_aborted () { return Poll :: Ready (Err (Aborted)) ; } Poll :: Pending } }
    };
}

impl_1343!();