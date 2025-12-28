macro_rules! deps {
    () => {
        Stream!();
        TryStream!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < S , T , E > TryStream for S where S : ? Sized + Stream < Item = Result < T , E > > , { type Ok = T ; type Error = E ; fn try_poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Self :: Ok , Self :: Error > > > { self . poll_next (cx) } }
    };
}

impl_22!()