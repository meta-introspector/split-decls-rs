macro_rules! deps {
    () => {
        Recv!();
        UnboundedReceiver!();
        RecvError!();
        TryRecvError!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < T > UnboundedReceiver < T > { # [doc = " Waits for a message from the channel."] # [doc = " If the channel is empty and closed, returns [`RecvError`]."] pub fn recv (& mut self) -> Recv < '_ , Self > { Recv :: new (self) } # [doc = " Closes the receiving half of a channel, without dropping it."] # [doc = ""] # [doc = " This prevents any further messages from being sent on the channel while"] # [doc = " still enabling the receiver to drain messages that are buffered."] pub fn close (& mut self) { if let Some (inner) = & mut self . inner { inner . set_closed () ; } } # [doc = " Tries to receive the next message without notifying a context if empty."] # [doc = ""] # [doc = " It is not recommended to call this function from inside of a future,"] # [doc = " only when you've otherwise arranged to be notified when the channel is"] # [doc = " no longer empty."] # [doc = ""] # [doc = " This function returns:"] # [doc = " * `Ok(Some(t))` when message is fetched"] # [doc = " * `Ok(None)` when channel is closed and no messages left in the queue"] # [doc = " * `Err(e)` when there are no messages available, but channel is not yet closed"] # [deprecated (note = "please use `try_recv` instead")] pub fn try_next (& mut self) -> Result < Option < T > , TryRecvError > { match self . next_message () { Poll :: Ready (msg) => Ok (msg) , Poll :: Pending => Err (TryRecvError :: Empty) , } } # [doc = " Tries to receive a message from the channel without blocking."] # [doc = " If the channel is empty, or empty and closed, this method returns an error."] pub fn try_recv (& mut self) -> Result < T , TryRecvError > { match self . next_message () { Poll :: Ready (Some (msg)) => Ok (msg) , Poll :: Ready (None) => Err (TryRecvError :: Closed) , Poll :: Pending => Err (TryRecvError :: Empty) , } } fn next_message (& mut self) -> Poll < Option < T > > { let inner = match self . inner . as_mut () { None => return Poll :: Ready (None) , Some (inner) => inner , } ; match unsafe { inner . message_queue . pop_spin () } { Some (msg) => { self . dec_num_messages () ; Poll :: Ready (Some (msg)) } None => { let state = decode_state (inner . state . load (SeqCst)) ; if state . is_closed () { self . inner = None ; Poll :: Ready (None) } else { Poll :: Pending } } } } fn dec_num_messages (& self) { if let Some (inner) = & self . inner { inner . state . fetch_sub (1 , SeqCst) ; } } }
    };
}

impl_87!()