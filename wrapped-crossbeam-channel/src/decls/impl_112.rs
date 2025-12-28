macro_rules! deps {
    () => {
        Timeout!();
        RecvTimeoutError!();
        Channel!();
        TryRecvError!();
        Token!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Channel { # [doc = " Creates a channel that delivers a message at a certain instant in time."] # [inline] pub (crate) fn new_deadline (when : Instant) -> Self { Self { delivery_time : when , received : AtomicBool :: new (false) , } } # [doc = " Attempts to receive a message without blocking."] # [inline] pub (crate) fn try_recv (& self) -> Result < Instant , TryRecvError > { if self . received . load (Ordering :: Relaxed) { return Err (TryRecvError :: Empty) ; } if Instant :: now () < self . delivery_time { return Err (TryRecvError :: Empty) ; } if ! self . received . swap (true , Ordering :: SeqCst) { Ok (self . delivery_time) } else { Err (TryRecvError :: Empty) } } # [doc = " Receives a message from the channel."] # [inline] pub (crate) fn recv (& self , deadline : Option < Instant >) -> Result < Instant , RecvTimeoutError > { if self . received . load (Ordering :: Relaxed) { utils :: sleep_until (deadline) ; return Err (RecvTimeoutError :: Timeout) ; } loop { let now = Instant :: now () ; let deadline = match deadline { _ if now >= self . delivery_time => break , Some (d) if now >= d => return Err (RecvTimeoutError :: Timeout) , Some (d) if d < self . delivery_time => d , _ => self . delivery_time , } ; thread :: sleep (deadline - now) ; } if ! self . received . swap (true , Ordering :: SeqCst) { Ok (self . delivery_time) } else { utils :: sleep_until (None) ; unreachable ! () } } # [doc = " Reads a message from the channel."] # [inline] pub (crate) unsafe fn read (& self , token : & mut Token) -> Result < Instant , () > { token . at . ok_or (()) } # [doc = " Returns `true` if the channel is empty."] # [inline] pub (crate) fn is_empty (& self) -> bool { if self . received . load (Ordering :: Relaxed) { return true ; } if Instant :: now () < self . delivery_time { return true ; } self . received . load (Ordering :: SeqCst) } # [doc = " Returns `true` if the channel is full."] # [inline] pub (crate) fn is_full (& self) -> bool { ! self . is_empty () } # [doc = " Returns the number of messages in the channel."] # [inline] pub (crate) fn len (& self) -> usize { usize :: from (! self . is_empty ()) } # [doc = " Returns the capacity of the channel."] # [inline] pub (crate) fn capacity (& self) -> Option < usize > { Some (1) } }
    };
}

impl_112!();