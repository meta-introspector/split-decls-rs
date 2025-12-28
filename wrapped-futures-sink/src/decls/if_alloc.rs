macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! if_alloc {
    () => {
        deps!();
        # [cfg (feature = "alloc")] mod if_alloc { use super :: * ; use core :: convert :: Infallible ; impl < T > Sink < T > for alloc :: vec :: Vec < T > { type Error = Infallible ; fn poll_ready (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : T) -> Result < () , Self :: Error > { unsafe { self . get_unchecked_mut () } . push (item) ; Ok (()) } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } } impl < T > Sink < T > for alloc :: collections :: VecDeque < T > { type Error = Infallible ; fn poll_ready (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : T) -> Result < () , Self :: Error > { unsafe { self . get_unchecked_mut () } . push_back (item) ; Ok (()) } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } } impl < S : ? Sized + Sink < Item > + Unpin , Item > Sink < Item > for alloc :: boxed :: Box < S > { type Error = S :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_ready (cx) } fn start_send (mut self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { Pin :: new (& mut * * self) . start_send (item) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut * * self) . poll_close (cx) } } }
    };
}

if_alloc!()