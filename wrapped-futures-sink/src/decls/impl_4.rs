macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < P , Item > Sink < Item > for Pin < P > where P : DerefMut + Unpin , P :: Target : Sink < Item > , { type Error = < P :: Target as Sink < Item > > :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . get_mut () . as_mut () . poll_ready (cx) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { self . get_mut () . as_mut () . start_send (item) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . get_mut () . as_mut () . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . get_mut () . as_mut () . poll_close (cx) } }
    };
}

impl_4!();