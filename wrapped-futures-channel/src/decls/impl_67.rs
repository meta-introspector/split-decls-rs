macro_rules! deps {
    () => {
        Receiver!();
        TrySendError!();
        Sender!();
        BoundedSenderInner!();
        SendError!();
        SendErrorKind!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > Sender < T > { # [doc = " Attempts to send a message on this `Sender`, returning the message"] # [doc = " if there was an error."] pub fn try_send (& mut self , msg : T) -> Result < () , TrySendError < T > > { if let Some (inner) = & mut self . 0 { inner . try_send (msg) } else { Err (TrySendError { err : SendError { kind : SendErrorKind :: Disconnected } , val : msg }) } } # [doc = " Send a message on the channel."] # [doc = ""] # [doc = " This function should only be called after"] # [doc = " [`poll_ready`](Sender::poll_ready) has reported that the channel is"] # [doc = " ready to receive a message."] pub fn start_send (& mut self , msg : T) -> Result < () , SendError > { self . try_send (msg) . map_err (| e | e . err) } # [doc = " Polls the channel to determine if there is guaranteed capacity to send"] # [doc = " at least one item without waiting."] # [doc = ""] # [doc = " # Return value"] # [doc = ""] # [doc = " This method returns:"] # [doc = ""] # [doc = " - `Poll::Ready(Ok(_))` if there is sufficient capacity;"] # [doc = " - `Poll::Pending` if the channel may not have"] # [doc = "   capacity, in which case the current task is queued to be notified once"] # [doc = "   capacity is available;"] # [doc = " - `Poll::Ready(Err(SendError))` if the receiver has been dropped."] pub fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , SendError > > { let inner = self . 0 . as_mut () . ok_or (SendError { kind : SendErrorKind :: Disconnected }) ? ; inner . poll_ready (cx) } # [doc = " Returns whether this channel is closed without needing a context."] pub fn is_closed (& self) -> bool { self . 0 . as_ref () . map (BoundedSenderInner :: is_closed) . unwrap_or (true) } # [doc = " Closes this channel from the sender side, preventing any new messages."] pub fn close_channel (& mut self) { if let Some (inner) = & mut self . 0 { inner . close_channel () ; } } # [doc = " Disconnects this sender from the channel, closing it if there are no more senders left."] pub fn disconnect (& mut self) { self . 0 = None ; } # [doc = " Returns whether the senders send to the same receiver."] pub fn same_receiver (& self , other : & Self) -> bool { match (& self . 0 , & other . 0) { (Some (inner) , Some (other)) => inner . same_receiver (other) , _ => false , } } # [doc = " Returns whether the sender send to this receiver."] pub fn is_connected_to (& self , receiver : & Receiver < T >) -> bool { match (& self . 0 , & receiver . inner) { (Some (inner) , Some (receiver)) => inner . is_connected_to (receiver) , _ => false , } } # [doc = " Hashes the receiver into the provided hasher"] pub fn hash_receiver < H > (& self , hasher : & mut H) where H : std :: hash :: Hasher , { use std :: hash :: Hash ; let ptr = self . 0 . as_ref () . map (| inner | inner . ptr ()) ; ptr . hash (hasher) ; } }
    };
}

impl_67!()