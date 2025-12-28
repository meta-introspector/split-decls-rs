macro_rules! deps {
    () => {
        Read!();
        Parts!();
        Rewind!();
        Write!();
        Result!();
        Upgraded!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl Upgraded { # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] pub (super) fn new < T > (io : T , read_buf : Bytes) -> Self where T : Read + Write + Unpin + Send + 'static , { Upgraded { io : Rewind :: new_buffered (Box :: new (io) , read_buf) , } } # [doc = " Tries to downcast the internal trait object to the type passed."] # [doc = ""] # [doc = " On success, returns the downcasted parts. On error, returns the"] # [doc = " `Upgraded` back."] pub fn downcast < T : Read + Write + Unpin + 'static > (self) -> Result < Parts < T > , Self > { let (io , buf) = self . io . into_inner () ; match io . __hyper_downcast () { Ok (t) => Ok (Parts { io : * t , read_buf : buf , }) , Err (io) => Err (Upgraded { io : Rewind :: new_buffered (io , buf) , }) , } } }
    };
}

impl_214!()