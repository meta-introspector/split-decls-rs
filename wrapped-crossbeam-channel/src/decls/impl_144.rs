macro_rules! deps {
    () => {
        TryRecvError!();
        Channel!();
        Timeout!();
        Token!();
        RecvTimeoutError!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Channel { # [doc = " Creates a channel that delivers messages periodically."] # [inline] pub (crate) fn new (delivery_time : Instant , dur : Duration) -> Self { Self { delivery_time : AtomicCell :: new (delivery_time) , duration : dur , } } # [doc = " Attempts to receive a message without blocking."] # [inline] pub (crate) fn try_recv (& self) -> Result < Instant , TryRecvError > { loop { let now = Instant :: now () ; let delivery_time = self . delivery_time . load () ; if now < delivery_time { return Err (TryRecvError :: Empty) ; } if self . delivery_time . compare_exchange (delivery_time , now + self . duration) . is_ok () { return Ok (delivery_time) ; } } } # [doc = " Receives a message from the channel."] # [inline] pub (crate) fn recv (& self , deadline : Option < Instant >) -> Result < Instant , RecvTimeoutError > { loop { let delivery_time = self . delivery_time . load () ; let now = Instant :: now () ; if let Some (d) = deadline { if d < delivery_time { if now < d { thread :: sleep (d - now) ; } return Err (RecvTimeoutError :: Timeout) ; } } if self . delivery_time . compare_exchange (delivery_time , delivery_time . max (now) + self . duration) . is_ok () { if now < delivery_time { thread :: sleep (delivery_time - now) ; } return Ok (delivery_time) ; } } } # [doc = " Reads a message from the channel."] # [inline] pub (crate) unsafe fn read (& self , token : & mut Token) -> Result < Instant , () > { token . tick . ok_or (()) } # [doc = " Returns `true` if the channel is empty."] # [inline] pub (crate) fn is_empty (& self) -> bool { Instant :: now () < self . delivery_time . load () } # [doc = " Returns `true` if the channel is full."] # [inline] pub (crate) fn is_full (& self) -> bool { ! self . is_empty () } # [doc = " Returns the number of messages in the channel."] # [inline] pub (crate) fn len (& self) -> usize { usize :: from (! self . is_empty ()) } # [doc = " Returns the capacity of the channel."] # [inline] pub (crate) fn capacity (& self) -> Option < usize > { Some (1) } }
    };
}

impl_144!()