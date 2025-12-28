macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < S : ? Sized + Sink < Item > + Unpin , Item > Sink < Item > for & mut S { type Error = S :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_ready (cx) } fn start_send (mut self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { Pin :: new (& mut * * self) . start_send (item) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_close (cx) } }
    };
}

impl_3!();