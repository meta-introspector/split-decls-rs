macro_rules! deps {
    () => {
        Kind!();
        Incoming!();
        Result!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Debug for Incoming { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (any (all (any (feature = "http1" , feature = "http2") , any (feature = "client" , feature = "server")) , feature = "ffi"))] # [derive (Debug)] struct Streaming ; # [derive (Debug)] struct Empty ; let mut builder = f . debug_tuple ("Body") ; match self . kind { Kind :: Empty => builder . field (& Empty) , # [cfg (any (all (any (feature = "http1" , feature = "http2") , any (feature = "client" , feature = "server")) , feature = "ffi"))] _ => builder . field (& Streaming) , } ; builder . finish () } }
    };
}

impl_27!();