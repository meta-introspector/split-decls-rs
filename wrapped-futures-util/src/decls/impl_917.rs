macro_rules! deps {
    () => {
        Sink!();
        Close!();
    };
}

macro_rules! impl_917 {
    () => {
        deps!();
        impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Close < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . sink) . poll_close (cx) } }
    };
}

impl_917!();