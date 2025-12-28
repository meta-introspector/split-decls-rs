macro_rules! deps {
    () => {
        Timeout!();
        Channel!();
        TryRecvError!();
        Token!();
        RecvTimeoutError!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < T > Channel < T > { # [doc = " Creates a channel that never delivers messages."] # [inline] pub (crate) fn new () -> Self { Self { _marker : PhantomData , } } # [doc = " Attempts to receive a message without blocking."] # [inline] pub (crate) fn try_recv (& self) -> Result < T , TryRecvError > { Err (TryRecvError :: Empty) } # [doc = " Receives a message from the channel."] # [inline] pub (crate) fn recv (& self , deadline : Option < Instant >) -> Result < T , RecvTimeoutError > { utils :: sleep_until (deadline) ; Err (RecvTimeoutError :: Timeout) } # [doc = " Reads a message from the channel."] # [inline] pub (crate) unsafe fn read (& self , _token : & mut Token) -> Result < T , () > { Err (()) } # [doc = " Returns `true` if the channel is empty."] # [inline] pub (crate) fn is_empty (& self) -> bool { true } # [doc = " Returns `true` if the channel is full."] # [inline] pub (crate) fn is_full (& self) -> bool { true } # [doc = " Returns the number of messages in the channel."] # [inline] pub (crate) fn len (& self) -> usize { 0 } # [doc = " Returns the capacity of the channel."] # [inline] pub (crate) fn capacity (& self) -> Option < usize > { Some (0) } }
    };
}

impl_139!();