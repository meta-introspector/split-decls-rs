macro_rules! deps {
    () => {
        Result!();
        Sender!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] impl fmt :: Debug for Sender { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [derive (Debug)] struct Open ; # [derive (Debug)] struct Closed ; let mut builder = f . debug_tuple ("Sender") ; match self . want_rx . peek () { watch :: CLOSED => builder . field (& Closed) , _ => builder . field (& Open) , } ; builder . finish () } }
    };
}

impl_29!();