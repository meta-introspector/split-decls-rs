macro_rules! deps {
    () => {
        Sink!();
        Flush!();
    };
}

macro_rules! impl_938 {
    () => {
        deps!();
        impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Flush < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . sink) . poll_flush (cx) } }
    };
}

impl_938!();